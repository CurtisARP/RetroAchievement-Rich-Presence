use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("RetroAchievements API error: {0}")]
    RaApi(String),

    #[error("Discord RPC error: {0}")]
    DiscordRpc(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Config file error: {0}")]
    ConfigIo(#[from] confy::ConfyError),
}