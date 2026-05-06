use std::str::FromStr;
use tokio::signal;
use tokio::time::{interval, Duration};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use confy::ConfyError;

mod config;
mod errors;
mod ra_api;
mod discord;
mod state;

use config::Config;
use errors::AppError;
use ra_api::RaClient;
use discord::DiscordPresence;
use state::AppState;

async fn run_app(config: Config) -> Result<(), AppError> {
    config.validate()
        .map_err(|e| AppError::Config(e))?;

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

fn load_config() -> Result<Config, AppError> {
    let cfg: Config = confy::load("ra-discord-rp", "config")
        .map_err(|e: ConfyError| AppError::ConfigIo(e))?;
    
    let mut cfg = cfg;
    cfg.load_from_env();
    
    if cfg.username.is_empty() || cfg.api_key.is_empty() || cfg.discord_client_id.is_empty() {
        let default_cfg = Config::default();
        confy::store("ra-discord-rp", "config", &default_cfg)
            .map_err(|e: ConfyError| AppError::ConfigIo(e))?;
        
        return Err(AppError::Config(
            "Missing credentials. Set RA_USERNAME, RA_API_KEY, and DISCORD_CLIENT_ID env vars.".to_string()
        ));
    }
    
    Ok(cfg)
}

fn parse_log_level(s: &str) -> Level {
    match s {
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}

#[tokio::main]
async fn main() {
    let log_level = match confy::load::<Config>("ra-discord-rp", "config") {
        Ok(cfg) => parse_log_level(&cfg.log_level),
        Err(_) => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run_app(config).await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}