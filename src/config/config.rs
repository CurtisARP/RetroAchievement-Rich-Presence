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
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.username.trim().is_empty() {
            errors.push("Username is required".to_string());
        }
        if self.api_key.trim().is_empty() {
            errors.push("RetroAchievements API key is required".to_string());
        }
        if self.discord_client_id.trim().is_empty() {
            errors.push("Discord client ID is required".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn ra_url(&self) -> String {
        format!(
            "{}?u={}&y={}&g={}&a={}",
            self.ra_base_url,
            self.username,
            self.api_key,
            self.ra_recent_games,
            self.ra_recent_achievements
        )
    }
}

pub fn load_config() -> Result<Config, ConfigError> {
    let config_path = super::paths::get_config_path()
        .ok_or(ConfigError::NoConfigPath)?;
    
    if !config_path.exists() {
        return Err(ConfigError::NotFound);
    }
    
    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| ConfigError::Io(e.to_string()))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| ConfigError::Parse(e.to_string()))?;
    
    config.validate()
        .map_err(|errors| ConfigError::Invalid(errors.join(", ")))?;
    
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), ConfigError> {
    let dir = super::paths::ensure_config_dir()
        .map_err(ConfigError::Io)?;
    let config_path = dir.join(super::paths::CONFIG_FILENAME);
    
    let content = toml::to_string_pretty(config)
        .map_err(|e| ConfigError::Serialize(e.to_string()))?;
    
    std::fs::write(&config_path, content)
        .map_err(|e| ConfigError::Io(e.to_string()))?;
    
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config file not found. Run with --setup to create one.")]
    NotFound,
    
    #[error("Could not determine config directory")]
    NoConfigPath,
    
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Failed to parse config: {0}")]
    Parse(String),
    
    #[error("Invalid config: {0}")]
    Invalid(String),
    
    #[error("Failed to serialize config: {0}")]
    Serialize(String),
}