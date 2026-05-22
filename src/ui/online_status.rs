use std::time::Duration;

use slint::ComponentHandle;
use tokio::time::{MissedTickBehavior, interval};

use crate::{
    integrations::{
        api::{RemoteConfig, wait_config},
        steam::SteamClient,
    },
    ui::AppWindow,
};

const REFRESH_INTERVAL: Duration = Duration::from_secs(15);

pub fn start(
    app: &AppWindow,
    runtime: &tokio::runtime::Runtime,
    steam: SteamClient,
    mut remote_config: RemoteConfig,
) {
    let app_weak = app.as_weak();

    runtime.spawn(async move {
        let Some(cfg) = wait_config(&mut remote_config).await else {
            return;
        };
        let appid = cfg.steam_appid;

        let mut ticker = interval(REFRESH_INTERVAL);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            ticker.tick().await;

            let text = match steam.current_players(appid).await {
                Ok(n) => n.to_string(),
                Err(err) => {
                    log::warn!("steam current_players: {err}");
                    "—".to_string()
                }
            };

            let result = app_weak.upgrade_in_event_loop(move |app| {
                app.set_online_count(text.into());
            });
            if result.is_err() {
                break;
            }
        }
    });
}
