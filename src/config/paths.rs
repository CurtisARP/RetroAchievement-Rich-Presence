use directories::ProjectDirs;
use std::path::PathBuf;

pub const APP_NAME: &str = "ra-discord-rp";
pub const CONFIG_FILENAME: &str = "config.toml";

pub fn get_config_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "retroachievements", APP_NAME)
        .map(|dirs| dirs.config_dir().to_path_buf())
}

pub fn get_config_path() -> Option<PathBuf> {
    get_config_dir().map(|dir| dir.join(CONFIG_FILENAME))
}

pub fn ensure_config_dir() -> Result<PathBuf, String> {
    let dir = get_config_dir().ok_or("Failed to determine config directory")?;
    
    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    Ok(dir)
}

pub fn get_config_dir_display() -> String {
    get_config_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}