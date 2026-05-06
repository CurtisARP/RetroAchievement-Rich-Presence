use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub username: String,
    pub api_key: String,

    #[serde(rename = "discord_client_id")]
    pub discord_client_id: String,

    #[serde(rename = "poll_interval_secs", default = "default_poll_interval")]
    pub poll_interval_secs: u64,

    #[serde(rename = "idle_timeout_secs", default = "default_idle_timeout")]
    pub idle_timeout_secs: u64,

    #[serde(rename = "log_level", default = "default_log_level")]
    pub log_level: String,

    #[serde(rename = "ra_base_url", default = "default_ra_base_url")]
    pub ra_base_url: String,

    #[serde(rename = "ra_recent_games", default = "default_recent_games")]
    pub ra_recent_games: u32,

    #[serde(rename = "ra_recent_achievements", default = "default_recent_achievements")]
    pub ra_recent_achievements: u32,

    #[serde(rename = "presence_buttons", default = "default_presence_buttons")]
    pub presence_buttons: bool,

    #[serde(rename = "presence_large_image_key", default)]
    pub presence_large_image_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            api_key: "".to_string(),
            discord_client_id: "".to_string(),
            poll_interval_secs: default_poll_interval(),
            idle_timeout_secs: default_idle_timeout(),
            log_level: default_log_level(),
            ra_base_url: default_ra_base_url(),
            ra_recent_games: default_recent_games(),
            ra_recent_achievements: default_recent_achievements(),
            presence_buttons: default_presence_buttons(),
            presence_large_image_key: None
        }       
    }   
}

fn default_poll_interval() -> u64 { 45 }

fn default_idle_timeout() -> u64 { 300 }

fn default_log_level() -> String { "info".to_string() }

fn default_ra_base_url() -> String { "https://retroachievements.org/API/API_GetUserSummary.php".to_string() }

fn default_recent_games() -> u32 { 5 }

fn default_recent_achievements() -> u32 { 5 }

fn default_presence_buttons() -> bool { true }

impl Config {   
    pub fn validate(&self) -> Result<(), String> {
        if self.username.is_empty() {            
            return Err("username is required (set RA_USERNAME env var)".to_string());
        }        
        if self.api_key.is_empty() {            
            return Err("api_key is required (set RA_API_KEY env var)".to_string());
        }        
        if self.discord_client_id.is_empty() {            
            return Err("discord_client_id is required (set DISCORD_CLIENT_ID env var)".to_string());       
        }        
        Ok(())
    }

    pub fn ra_url(&self) -> String { format!(
            "{}?u={}&y={}&g={}&a={}",
            self.ra_base_url, self.username, self.api_key, self.ra_recent_games, self.ra_recent_achievements
        )
    }

    pub fn load_from_env(&mut self) {
        if let Ok(v) = std::env::var("RA_USERNAME") {
            if !v.is_empty() { self.username = v; }
        }
        if let Ok(v) = std::env::var("RA_API_KEY") {
            if !v.is_empty() { self.api_key = v; }
        }
        if let Ok(v) = std::env::var("DISCORD_CLIENT_ID") {
            if !v.is_empty() { self.discord_client_id = v; }
        }
    }
}