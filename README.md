# CS2shock-XPERIMENT OpenShock Support

# THIS CODE IS UNAFFILIATED WITH THE REAL VERSION BY VolcanoCookies
# THIS IS A MODIFIED VERSION WITH THE PURPOSE OF USING OpenShock INSTEAD OF PiShock
# THEIR CODE IS NOT AS SHIT AS THE CODE IN THIS REPO (I THINK)
# THIS CODE IS A JERRYRIGED VERSION AND IS EXPERIMENTAL AND COMES WITH **NO SUPPORT**

[![CI](https://github.com/NanashiTheNameless/cs2shock-XPERIMENT/actions/workflows/ci.yml/badge.svg)](https://github.com/NanashiTheNameless/cs2shock-XPERIMENT/actions/workflows/ci.yml)
[![Release](https://github.com/NanashiTheNameless/cs2shock-XPERIMENT/actions/workflows/release.yml/badge.svg)](https://github.com/NanashiTheNameless/cs2shock-XPERIMENT/actions/workflows/release.yml)

---

## ⚠️ CRITICAL DISCLAIMER: AI-GENERATED CODE AHEAD ⚠️

> **Listen up, dear user.** This codebase was lovingly ~~Buttfucked~~\*_ahem_\*Tweaked by an AI – you know, those super reliable entities that are *totally* known for writing production-ready, bug-free code. So naturally, you should trust it implicitly with your physical wellbeing and nervous system. What could possibly go wrong?

**In case the sarcasm wasn't thick enough, let me spell it out:**

- 🤖 This was written by an AI. An AI that has never actually *used* OpenShock hardware.
- 🤷 The AI has no concept of pain, discomfort, or why shocking yourself for dying in a video game might be questionable life choices.
- 🎲 The code *might* work. Or it might shock you at random intervals. Or continuously. Who knows? Debugging is for humans!
- 🔥 If this burns down your house, shocks your cat, or causes your OpenShock to achieve sentience and start a robot uprising, that's on YOU buddy.
- 📖 The AI read the API docs once. *Once.* And probably hallucinated half of them.
- 🧂 Take everything here with a grain of salt the size of Jupiter. Actually, make that Saturn – it has better rings.

**Legal Department's Input (they're screaming):**
- ⚖️ Use this code entirely at your own risk and peril
- 🏥 We are not responsible for any injuries, discomfort, or existential dread
- 💀 Seriously, test this thoroughly before using it on yourself
- 🧪 Maybe start with the lowest possible settings. Or better yet, don't use it at all.
- 🎯 The AI's idea of "safe defaults" is... let's call it "optimistic"

**Reality Check:**
- This code interfaces with hardware that delivers electrical shocks
- To your body
- Based on video game events
- Written by a language model
- Are you *really* sure about this?

**Advanced Disclaimer:**
- If you're reading this and thinking "the AI is being too cautious," *you are the problem*
- If you're already in pain from using this, perhaps reconsider your life choices
- If this works perfectly for you, please tell us how, because the AI certainly doesn't know

Remember: **Friends don't let friends run untested AI code connected to shock devices.** But hey, I'm just text on a screen. You do you. 🤡

---

## How it works

Simple, you get shocked when you die during a live match (so not warmup).

Matches do not need to be premier or comp, can be any match.

This project uses **OpenShock** (not PiShock). You'll need an OpenShock account and device.

There are two modes of zapping, either random that picks a value between your configured min and max, everytime you die. Or LastHitPercentage which takes a value depending on the percentage of health you had before you died, so if you had 25hp and died you will be zapped for 25% of your configured max.

There are also two options to beep whenever a match starts, and whenever a round starts. For you forgetful folks.

## Download

Pre-built binaries are available for Windows in the [Releases](https://github.com/VolcanoCookies/cs2shock/releases) page.

Download:
- **Windows x64**: `cs2shock-windows-x64.zip`

*Linux and macOS support has been abandoned because let's be real, you're playing CS2 on Windows anyway.*

## Usage

1. Find your install directory, to do so go to steam, right click on Counter-Strike 2, go to `Manage > Browse Local Files`.
2. Put `gamestate_integration_cs2shock.cfg` in the `game/csgo/cfg` folder. (NOT THE `csgo/cfg` folder). You can now close this folder.
3. You can now run `cs2shock.exe`.

Once you save your settings once, a `config.json` file will be placed next to `cs2shock.exe`.

### Configuration

The application requires OpenShock API credentials:

- **Shocker ID**: Your OpenShock shocker UUID (found in your OpenShock dashboard)
- **API Key**: Your OpenShock API token (create one in your account settings)

You can obtain these from your OpenShock account at [OpenShock](https://openshock.app/).

**Important**: Duration values are in **milliseconds** (300-30000ms), not seconds.

Example `config.json`:
```json
{
  "shock_mode": "LastHitPercentage",
  "min_duration": 500,
  "max_duration": 3000,
  "min_intensity": 15,
  "max_intensity": 83,
  "beep_on_match_start": false,
  "beep_on_round_start": true,
  "apikey": "your-openshock-api-token-here",
  "shocker_id": "your-shocker-uuid-here"
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
