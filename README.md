# RetroAchievements Discord Rich Presence

A lightweight Rust application that displays your RetroAchievements.org activity as Discord Rich Presence.

## Features

- Automatically shows what retro game you're playing on Discord
- Displays game box art, title, and console information
- Optional profile and achievements buttons
- Credentials stored locally in platform-specific config directory
- Minimal resource usage with async Rust

## Quick Start

### 1. Install

Download a [release binary](https://github.com/yourusername/ra-discord-rp/releases) or build from source:

```bash
git clone https://github.com/yourusername/ra-discord-rp.git
cd ra-discord-rp
cargo build --release
```

### 2. Run Setup

On first run, the interactive setup will guide you through configuration:

```bash
./target/release/ra-discord-rp --setup
```

You'll need:
- **RetroAchievements Username** - Your RA username
- **RetroAchievements API Key** - Get it from https://retroachievements.org/controlpanel
- **Discord Client ID** - Create an app at https://discord.com/developers/applications

### 3. Run the App

```bash
./target/release/ra-discord-rp
```

The app will poll RetroAchievements every 45 seconds and update your Discord status when you're playing games.

## CLI Commands

```
ra-discord-rp [OPTIONS]

Options:
  --setup          Run interactive setup to create/reconfigure
  --config-path    Show config file location
  --version        Show version
  --help           Show this help message
```

## Configuration

Config is stored in platform-specific locations:

- **Linux**: `~/.config/ra-discord-rp/config.toml`
- **macOS**: `~/Library/Application Support/ra-discord-rp/config.toml`
- **Windows**: `%APPDATA%\ra-discord-rp\config.toml`

### Config Options

```toml
username = "your_username"
api_key = "your_api_key"
discord_client_id = "your_discord_client_id"

# How often to poll RA API (seconds)
poll_interval_secs = 45

# Idle timeout before clearing presence (seconds)
idle_timeout_secs = 300

# Log level: debug, info, warn, error
log_level = "info"

# Number of recent games to fetch
ra_recent_games = 5

# Number of recent achievements to fetch
ra_recent_achievements = 5

# Show buttons in Discord presence
presence_buttons = true

# Custom large image key (optional)
presence_large_image_key = null
```

## Requirements

- Discord desktop app running
- RetroAchievements.org account
- Discord application with Rich Presence enabled

## Troubleshooting

**"No configuration found"**
→ Run `ra-discord-rp --setup` to create a config file

**"Failed to validate credentials"**
→ Check your API key at https://retroachievements.org/controlpanel
→ Verify your Discord Client ID is correct

**Discord presence not updating**
→ Ensure Discord desktop app is running
→ Check the Discord app has Rich Presence enabled
→ View logs with `log_level = "debug"` in config

**Config location**
→ Run `ra-discord-rp --config-path` to see where config is stored

## Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run

# Run setup
cargo run -- --setup
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Pull requests welcome! Please ensure:
- `cargo fmt` has been run
- `cargo clippy` passes without warnings
- Tests pass (if applicable)
