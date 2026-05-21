use std::time::Duration;

use reqwest::Client;
use serde::Deserialize;
use tokio::sync::watch;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

const CONFIG_URL: &str =
    "https://raw.githubusercontent.com/Bezdarnost01/NewLauncher/main/assets/remote/config.json";

pub type RemoteConfig = watch::Receiver<Option<ApiConfig>>;
pub type RemoteConfigSender = watch::Sender<Option<ApiConfig>>;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub version: String,
    pub last_update_date: String,
    pub steam_appid: u32,
    pub telegram_link: String,
    pub discord_link: String
}

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        let http = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to build HTTP client");

        Self { http }
    }

    pub async fn get_config(&self) -> Result<ApiConfig, reqwest::Error> {
        self.http
            .get(CONFIG_URL)
            .send()
            .await?
            .error_for_status()?
            .json::<ApiConfig>()
            .await
    }
}

pub fn config_channel() -> (RemoteConfigSender, RemoteConfig) {
    watch::channel(None)
}

pub async fn wait_config(remote_config: &mut RemoteConfig) -> Option<ApiConfig> {
    loop {
        if let Some(config) = remote_config.borrow().clone() {
            return Some(config);
        }

        if remote_config.changed().await.is_err() {
            return None;
        }
    }
}