use std::time::Duration;

use serde::Deserialize;
use std::error::Error;
use tokio::sync::watch;

use crate::{
    config::{Config, ensure_backgrounds_dir},
    integrations::http::HttpClient,
};

const CONFIG_URL: &str =
    "https://raw.githubusercontent.com/Bezdarnost01/NewLauncher/main/assets/remote/config.json";

const MAX_ATTEMPTS: usize = 5;
const INITIAL_BACKOFF: Duration = Duration::from_secs(2);
const MAX_BACKOFF: Duration = Duration::from_secs(60);

pub type RemoteConfig = watch::Receiver<Option<ApiConfig>>;
pub type RemoteConfigSender = watch::Sender<Option<ApiConfig>>;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub version: String,
    pub last_update_date: String,
    pub steam_appid: u32,
    pub telegram_link: String,
    pub discord_link: String,
    pub backgrounds: Vec<ApiBackground>
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiBackground {
    pub name: String,
    pub url: String,
    pub file: String,
}

#[derive(Clone)]
pub struct ApiClient {
    http: HttpClient,
}

impl ApiClient {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn fetch_config(&self) -> Option<ApiConfig> {
        let mut backoff = INITIAL_BACKOFF;

        for attempt in 1..=MAX_ATTEMPTS {
            match self.try_fetch_config().await {
                Ok(cfg) => return Some(cfg),
                Err(err) => {
                    log::warn!(
                        "remote config attempt {attempt}/{MAX_ATTEMPTS} failed: {err}"
                    );
                    if attempt < MAX_ATTEMPTS {
                        tokio::time::sleep(backoff).await;
                        backoff = (backoff * 2).min(MAX_BACKOFF);
                    }
                }
            }
        }

        None
    }

    async fn try_fetch_config(&self) -> Result<ApiConfig, reqwest::Error> {
        self.http
            .raw()
            .get(CONFIG_URL)
            .send()
            .await?
            .error_for_status()?
            .json::<ApiConfig>()
            .await
    }

    pub async fn download_backgrounds(
        &self,
        config: &Config,
        remote_config: &ApiConfig,
    ) -> Result<(), Box<dyn Error + Send + Sync>>  {
        if !config.background_enabled {
            return Ok(());
        }

        let background_dir = ensure_backgrounds_dir()?;

        for background in &remote_config.backgrounds {
            let file_path = background_dir.join(&background.file);

            if file_path.exists() {
                continue;
            }

            let bytes = self
                .http
                .raw()
                .get(&background.url)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?;
                
            tokio::fs::write(&file_path, bytes).await?;
        }
        Ok(())
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