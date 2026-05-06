pub mod config;
pub mod paths;
pub mod setup;

pub use config::{Config, load_config, save_config, ConfigError};
pub use paths::{get_config_path, get_config_dir_display};