use std::sync::Arc;

use log::{debug, error, info};
use serde::Serialize;
use tokio::sync::RwLock;

use crate::config::Config;

pub async fn shock(config: Arc<RwLock<Config>>, intensity: i32, duration: i32) {
    debug!(target: "OpenShock API", "Sending shock: intensity={}, duration={}s", intensity, duration);

    let res = post(
        config,
        OpenShockOp::Shock {
            intensity,
            duration,
        },
    )
    .await;

    match res {
        Ok(_) => {
            info!(target: "OpenShock API",
                "Successfully sent shock (intensity: {}, duration: {}s)", intensity, duration
            );
        }
        Err(e) => {
            error!(target: "OpenShock API", "Failed to send shock: {}", e);
        }
    }
}

pub async fn vibrate(config: Arc<RwLock<Config>>, intensity: i32, duration: i32) {
    debug!(target: "OpenShock API",
        "Sending vibrate: intensity={}, duration={}s", intensity, duration
    );

    let res = post(
        config,
        OpenShockOp::Vibrate {
            intensity,
            duration,
        },
    )
    .await;

    match res {
        Ok(_) => {
            info!(
                target: "OpenShock API",
                "Successfully sent vibrate (intensity: {}, duration: {}s)", intensity, duration
            );
        }
        Err(e) => {
            error!(target: "OpenShock API", "Failed to send vibrate: {}", e);
        }
    }
}

pub async fn beep(config: Arc<RwLock<Config>>, duration: i32) {
    debug!(target: "OpenShock API", "Sending beep: duration={}s", duration);

    let res = post(config, OpenShockOp::Beep { duration }).await;

    match res {
        Ok(_) => {
            info!(
                target: "OpenShock API",
                "Successfully sent beep (duration: {}s)", duration
            );
        }
        Err(e) => {
            error!(target: "OpenShock API", "Failed to send beep: {}", e);
        }
    }
}

pub async fn post(config: Arc<RwLock<Config>>, op: OpenShockOp) -> Result<i32, String> {
    let config = config.read().await;
    
    // Convert duration from seconds to milliseconds
    let (endpoint, body) = match op {
        OpenShockOp::Beep { duration } => {
            let body = OpenShockRequest {
                intensity: 0,
                duration: duration * 1000, // Convert to milliseconds
            };
            ("beep", body)
        }
        OpenShockOp::Vibrate { intensity, duration } => {
            let body = OpenShockRequest {
                intensity,
                duration: duration * 1000, // Convert to milliseconds
            };
            ("vibrate", body)
        }
        OpenShockOp::Shock { intensity, duration } => {
            let body = OpenShockRequest {
                intensity,
                duration: duration * 1000, // Convert to milliseconds
            };
            ("shock", body)
        }
    };

    let url = format!(
        "https://api.openshock.app/2/shockers/{}/control/{}",
        config.shocker_id, endpoint
    );

    let res = reqwest::Client::new()
        .post(&url)
        .header("OpenShockToken", &config.api_token)
        .header("User-Agent", "CS2Shock/1.1.0")
        .json(&body)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                return Ok(res.status().as_u16() as i32);
            } else {
                let status = res.status().as_u16();
                let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Err(format!(
                    "Failed to post to OpenShock: {} - {}",
                    status, error_text
                ));
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct OpenShockRequest {
    intensity: i32,
    duration: i32,
}

#[derive(Debug, Clone)]
pub enum OpenShockOp {
    Beep { duration: i32 },
    Vibrate { intensity: i32, duration: i32 },
    Shock { intensity: i32, duration: i32 },
}
