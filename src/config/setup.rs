use dialoguer::{Input, Confirm, Select};
use crate::config::{Config, save_config, ConfigError};
use crate::ra_api::RaClient;

pub fn run_setup() -> Result<Config, SetupError> {
    println!("\n=== RetroAchievements Discord Rich Presence Setup ===\n");
    
    let mut config = Config::default();
    
    config.username = get_username()?;
    config.api_key = get_api_key()?;
    config.discord_client_id = get_discord_client_id()?;
    
    if let Ok(Some(console)) = select_console() {
        config.presence_large_image_key = Some(console);
    }
    
    let buttons_enabled = Confirm::new()
        .with_prompt("Enable Discord buttons (Profile, Achievements)?")
        .default(true)
        .interact()
        .map_err(|e| SetupError::Io(e.to_string()))?;
    config.presence_buttons = buttons_enabled;
    
    println!("\nValidating credentials...");
    if let Err(e) = validate_credentials(&config) {
        println!("\n⚠️  Warning: Could not validate credentials with RetroAchievements API.");
        println!("   Error: {}", e);
        println!("   Your config will be saved, but please verify the credentials are correct.\n");
        
        let save_anyway = Confirm::new()
            .with_prompt("Save anyway?")
            .default(false)
            .interact()
            .map_err(|e| SetupError::Io(e.to_string()))?;
        
        if !save_anyway {
            return Err(SetupError::ValidationFailed);
        }
    } else {
        println!("✓ Credentials validated successfully!\n");
    }
    
    save_config(&config)
        .map_err(|e: ConfigError| SetupError::SaveFailed(e.to_string()))?;
    
    println!("✓ Configuration saved!\n");
    println!("Run the application with: cargo run\n");
    
    Ok(config)
}

fn get_username() -> Result<String, SetupError> {
    loop {
        let input: String = Input::new()
            .with_prompt("RetroAchievements Username")
            .interact()
            .map_err(|e| SetupError::Io(e.to_string()))?;
        
        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("Username cannot be empty. Please try again.\n");
            continue;
        }
        
        return Ok(trimmed.to_string());
    }
}

fn get_api_key() -> Result<String, SetupError> {
    println!("\nTo get your API key:");
    println!("  1. Go to https://retroachievements.org/controlpanel");
    println!("  2. Scroll to 'API' section");
    println!("  3. Click 'Generate' to create a new API key\n");
    
    loop {
        let input: String = Input::new()
            .with_prompt("RetroAchievements API Key")
            .interact()
            .map_err(|e| SetupError::Io(e.to_string()))?;
        
        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("API key cannot be empty. Please try again.\n");
            continue;
        }
        
        if trimmed.len() < 10 {
            println!("API key seems too short. Please try again.\n");
            continue;
        }
        
        return Ok(trimmed.to_string());
    }
}

fn get_discord_client_id() -> Result<String, SetupError> {
    println!("\nTo get your Discord Client ID:");
    println!("  1. Go to https://discord.com/developers/applications");
    println!("  2. Create a new application (or select existing)");
    println!("  3. Copy the 'Application ID' from the general settings\n");
    
    loop {
        let input: String = Input::new()
            .with_prompt("Discord Application Client ID")
            .interact()
            .map_err(|e| SetupError::Io(e.to_string()))?;
        
        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("Client ID cannot be empty. Please try again.\n");
            continue;
        }
        
        if !trimmed.chars().all(|c| c.is_ascii_digit()) {
            println!("Client ID should only contain digits. Please try again.\n");
            continue;
        }
        
        return Ok(trimmed.to_string());
    }
}

fn select_console() -> Result<Option<String>, SetupError> {
    let consoles = vec![
        "None (use game box art)",
        "GameCube",
        "PlayStation 2",
        "PlayStation",
        "Xbox",
        "Xbox 360",
        "NES",
        "SNES",
        "Nintendo 64",
        "Game Boy Advance",
        "Game Boy Color",
        "Game Boy",
        "Sega Genesis",
        "Sega Saturn",
        "Dreamcast",
        "Wii",
        "Wii U",
        "Nintendo Switch",
        "Arcade",
    ];
    
    let selection = Select::new()
        .with_prompt("Select a default console image (optional)")
        .items(&consoles)
        .default(0)
        .interact()
        .map_err(|e| SetupError::Io(e.to_string()))?;
    
    if selection == 0 {
        Ok(None)
    } else {
        Ok(Some(consoles[selection].to_lowercase().replace(" ", "")))
    }
}

fn validate_credentials(config: &Config) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async {
        let client = RaClient::new(config.clone());
        client.fetch_user_summary().await
    }).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    #[error("Failed to validate credentials")]
    ValidationFailed,
    
    #[error("Failed to save config: {0}")]
    SaveFailed(String),
    
    #[error("IO error: {0}")]
    Io(String),
}