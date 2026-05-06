use presenceforge::ActivityBuilder;
use presenceforge::sync::DiscordIpcClient;
use crate::ra_api::UserSummary;
use crate::config::Config;
use crate::errors::AppError;

pub struct DiscordPresence {
    client: DiscordIpcClient,
    config: Config,
}

fn console_to_image_key(console_name: &str) -> Option<&'static str> {
    match console_name {
        "GameCube" => Some("gamecube"),
        "PlayStation 2" => Some("ps2"),
        "PlayStation" => Some("ps1"),
        "Xbox" => Some("xbox"),
        "Xbox 360" => Some("xbox360"),
        "NES" => Some("nes"),
        "SNES" => Some("snes"),
        "Nintendo 64" => Some("n64"),
        "Game Boy Advance" => Some("gba"),
        "Game Boy Color" => Some("gbc"),
        "Game Boy" => Some("gb"),
        "Sega Genesis" => Some("genesis"),
        "Sega Saturn" => Some("saturn"),
        "Dreamcast" => Some("dreamcast"),
        "Wii" => Some("wii"),
        "Wii U" => Some("wiiu"),
        "Nintendo Switch" => Some("switch"),
        "Arcade" => Some("arcade"),
        _ => None,
    }
}

impl DiscordPresence {
    pub fn new(config: Config) -> Result<Self, AppError> {
        let client = DiscordIpcClient::new(&config.discord_client_id)
            .map_err(|e| AppError::DiscordRpc(e.to_string()))?;

        Ok(Self { client, config })
    }

    pub fn connect(&mut self) -> Result<(), AppError> {
        self.client.connect()
            .map_err(|e| AppError::DiscordRpc(e.to_string()))?;
        Ok(())
    }

    pub fn set_presence(&mut self, user_summary: &UserSummary) -> Result<(), AppError> {
        let current_game = match user_summary.get_current_game() {
            Some(game) => game,
            None => {
                return self.clear_presence();
            }
        };

        let mut activity = ActivityBuilder::new();

        if let Some(title) = Some(&current_game.title) {
            activity = activity.details(title.as_str());
        }

        if let Some(rp_msg) = &user_summary.rich_presence_msg {
            if !rp_msg.is_empty() {
                activity = activity.state(rp_msg.as_str());
            }
        }

        // Try to use RA game image URL directly
        if let Some(image_url) = current_game.get_game_image_url() {
            activity = activity.large_image(&image_url);
            // Console name as hover text
            if !current_game.console_name.is_empty() {
                activity = activity.large_text(&current_game.console_name);
            }
        } else if let Some(ref image_key) = self.config.presence_large_image_key {
            // Fall back to custom asset key
            activity = activity.large_image(image_key);
        } else if let Some(image_key) = console_to_image_key(&current_game.console_name) {
            // Fall back to console-based asset key
            activity = activity.large_image(image_key);
        }

        if self.config.presence_buttons {
            let ra_profile_url = format!(
                "https://retroachievements.org/user/{}",
                user_summary.user
            );
            
            activity = activity
                .button("View Profile", &ra_profile_url)
                .button("Achievements", "https://retroachievements.org/");
        }

        let built = activity.build();
        
        self.client.set_activity(&built)
            .map_err(|e| AppError::DiscordRpc(e.to_string()))?;

        Ok(())
    }

    pub fn clear_presence(&mut self) -> Result<(), AppError> {
        self.client.clear_activity()
            .map_err(|e| AppError::DiscordRpc(e.to_string()))?;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), AppError> {
        let _ = self.clear_presence();
        Ok(())
    }
}