use h_mail_interface::server_config::ServerConfig;
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;

pub static CONFIG: Lazy<ServerConfig> = Lazy::new(|| {
    let config_file = PathBuf::from_str("config.json").unwrap();
    if config_file.is_file() {
        let x = serde_json::from_str(&fs::read_to_string(&config_file).unwrap()).unwrap();
        info!("Loaded config file: {}", config_file.display());
        x
    } else {
        let default = ServerConfig::default();
        let config_string = serde_json::to_string_pretty(&default).unwrap();
        fs::write(&config_file, config_string).unwrap();
        info!("Created default config file: {}", config_file.display());
        default
    }
});
