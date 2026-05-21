use std::time::Duration;

use crate::{
    integrations::{
        api::{RemoteConfig, wait_config},
        steam::SteamClient,
    },
    ui::AppWindow,
};

use slint::ComponentHandle;
use tokio::time::{interval, MissedTickBehavior};

const REFRESH_INTERVAL: Duration = Duration::from_secs(15);

pub fn start(
    app: &AppWindow,
    runtime: &tokio::runtime::Runtime,
    mut remote_config: RemoteConfig,
) {
    let steam = SteamClient::new();
    let app_weak = app.as_weak();

    runtime.spawn(async move {
        let Some(config) = wait_config(&mut remote_config).await else {
            return;
        };

        let steam_appid = config.steam_appid;

        let mut ticker = interval(REFRESH_INTERVAL);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            ticker.tick().await;

            let online_count = steam.current_players(steam_appid).await;

            let result = app_weak.upgrade_in_event_loop(move |app| {
                app.set_online_count(online_count as i32);
            });

            if result.is_err() {
                break;
            }
        }
    });
}