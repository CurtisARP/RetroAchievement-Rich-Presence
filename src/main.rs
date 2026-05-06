use tokio::signal;
use tokio::time::{interval, Duration};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod errors;
mod ra_api;
mod discord;
mod state;

use config::{Config, load_config, get_config_path, get_config_dir_display};
use config::setup::run_setup;
use config::config::ConfigError;
use errors::AppError;
use ra_api::RaClient;
use discord::DiscordPresence;
use state::AppState;

fn setup_logging(log_level: &str) {
    let level = match log_level {
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

async fn run_app(config: Config) -> Result<(), AppError> {
    config.validate()
        .map_err(|e: Vec<String>| AppError::Config(e.join("\n")))?;

    info!("Connecting to Discord...");
    let mut discord = DiscordPresence::new(config.clone())?;
    discord.connect()?;
    info!("Connected to Discord");

    let ra_client = RaClient::new(config.clone());
    let mut app_state = AppState::new(config.idle_timeout_secs);
    let mut poll_interval = interval(Duration::from_secs(config.poll_interval_secs));

    info!("Starting polling loop (interval: {}s)", config.poll_interval_secs);
    
    loop {
        tokio::select! {
            _ = poll_interval.tick() => {
                match ra_client.fetch_user_summary().await {
                    Ok(user_summary) => {
                        let is_playing = user_summary.is_playing();
                        app_state.update_playing(is_playing);

                        if is_playing {
                            info!("Playing: {}", user_summary.get_current_game()
                                .map(|g| g.title.clone())
                                .unwrap_or_else(|| "Unknown".to_string()));
                            
                            if let Err(e) = discord.set_presence(&user_summary) {
                                error!("Failed to update presence: {}", e);
                            }
                        } else if app_state.should_clear() {
                            info!("Idle timeout reached, clearing presence");
                            if let Err(e) = discord.clear_presence() {
                                error!("Failed to clear presence: {}", e);
                            }
                            app_state.reset();
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch user summary: {}", e);
                    }
                }
            }
            _ = signal::ctrl_c() => {
                info!("Shutting down...");
                break;
            }
        }
    }

    let _ = discord.shutdown();
    info!("Shutdown complete");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }
    
    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("ra-discord-rp v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    if args.iter().any(|a| a == "--config-path") {
        match get_config_path() {
            Some(path) => {
                println!("Config location: {}", path.to_string_lossy());
            }
            None => {
                println!("Could not determine config path");
            }
        }
        return;
    }
    
    if args.iter().any(|a| a == "--setup") {
        match run_setup() {
            Ok(_config) => {
                println!("Setup complete! You can now run the application.");
            }
            Err(e) => {
                eprintln!("Setup failed: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }
    
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(ConfigError::NotFound) => {
            eprintln!("No configuration found.");
            eprintln!("Run with --setup to create a configuration file.");
            eprintln!("Config location: {}", get_config_dir_display());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            eprintln!("Run with --setup to reconfigure.");
            std::process::exit(1);
        }
    };
    
    setup_logging(&config.log_level);
    
    if let Err(e) = tokio::runtime::Runtime::new().unwrap().block_on(run_app(config)) {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn print_help() {
    println!("RetroAchievements Discord Rich Presence");
    println!();
    println!("Usage: ra-discord-rp [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --setup          Run interactive setup to create config");
    println!("  --config-path    Show config file location");
    println!("  --version        Show version");
    println!("  --help           Show this help message");
    println!();
    println!("Config location:");
    println!("  Linux:   ~/.config/ra-discord-rp/config.toml");
    println!("  macOS:   ~/Library/Application Support/ra-discord-rp/config.toml");
    println!("  Windows: %APPDATA%\\ra-discord-rp\\config.toml");
}