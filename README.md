# CS2shock

## How it works

Simple, you get shocked when you die during a live match (so not warmup).

Matches do not need to be premier or comp, can be any match.

There are two modes of zapping, either random that picks a value between your configured min and max, everytime you die. Or LastHitPercentage which takes a value depending on the percentage of health you had before you died, so if you had 25hp and died you will be zapped for 25% of your configured max.

There are also two options to beep whenever a match starts, and whenever a round starts. For you forgetful folks.

## Usage

1. Find your install directory, to do so go to steam, right click on Counter-Strike 2, go to `Manage > Browse Local Files`.
2. Put `gamestate_integration_cs2shock.cfg` in the `game/csgo/cfg` folder. (NOT THE `csgo/cfg` folder). You can now close this folder.
3. You can now run `cs2shock.exe`.

Once you save your settings once, a `config.json` file will be placed next to `cs2shock.exe`.

### Configuration

The application requires OpenShock API credentials:

- **Shocker ID**: Your OpenShock shocker/device ID
- **API Token**: Your OpenShock API token

You can obtain these from your OpenShock account at [OpenShock](https://openshock.app/).

Example `config.json`:
```json
{
  "shock_mode": "LastHitPercentage",
  "min_duration": 5,
  "max_duration": 15,
  "min_intensity": 15,
  "max_intensity": 83,
  "beep_on_match_start": false,
  "beep_on_round_start": true,
  "shocker_id": "your-shocker-id-here",
  "api_token": "your-api-token-here"
}
```

## Building from source

### Prerequisites

-   [Rust installed](https://doc.rust-lang.org/cargo/getting-started/installation.html)

1. Clone the repository
    - `git clone https://github.com/VolcanoCookies/cs2shock.git`
2. Open the created folder
    - `cd cs2shock`
3. Build the project
    - `cargo build --release`

You can then find the executable in `cs2shock/target/release/cs2shock.exe`
