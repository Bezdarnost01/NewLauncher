use log::warn;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

fn default_true() -> bool {
    true
}

fn default_game_folder() -> String {
    String::new()
}

fn default_background_name() -> String {
    String::new()
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Config {
    #[serde(default = "default_game_folder")]
    pub game_folder: String,

    #[serde(default = "default_true")]
    pub discord_rpc_enabled: bool,

    #[serde(default = "default_true")]
    pub anti_cheat_enabled: bool,

    #[serde(default = "default_true")]
    pub match_alert_enabled: bool,

    #[serde(default = "default_true")]
    pub background_enabled: bool,

    #[serde(default = "default_true")]
    pub random_backgrounds_enabled: bool,

    #[serde(default = "default_background_name")]
    pub background_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_folder: default_game_folder(),
            discord_rpc_enabled: default_true(),
            anti_cheat_enabled: default_true(),
            match_alert_enabled: default_true(),
            background_enabled: default_true(),
            random_backgrounds_enabled: default_true(),
            background_name: default_background_name(),
        }
    }
}

pub fn load_config() -> Config {
    let path = config_path();

    match fs::read_to_string(&path) {
        Ok(config_text) => match serde_json::from_str(&config_text) {
            Ok(config) => config,
            Err(err) => {
                warn!("Failed to parse config file {}: {err}", path.display());
                Config::default()
            }
        },
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            let config = Config::default();

            if let Err(err) = save_config(&config) {
                warn!("Failed to create config file {}: {err}", path.display());
            }

            config
        }
        Err(err) => {
            warn!("Failed to read config file {}: {err}", path.display());
            Config::default()
        }
    }
}

pub fn app_data_dir() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("DBDLegacy442")
}

pub fn backgrounds_dir() -> PathBuf {
    app_data_dir().join("backgrounds")
}

fn config_path() -> PathBuf {
    app_data_dir().join("config.json")
}

pub fn ensure_backgrounds_dir() -> io::Result<PathBuf> {
    let path = backgrounds_dir();
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let path = config_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config_text = serde_json::to_string_pretty(config).map_err(io::Error::other)?;
    fs::write(path, config_text)
}