use std::time::Duration;

use crate::{
    integrations::{
        api::RemoteConfig,
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
        let steam_appid = match wait_steam_appid(&mut remote_config).await {
            Some(appid) => appid,
            None => return,
        };

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

async fn wait_steam_appid(remote_config: &mut RemoteConfig) -> Option<u32> {
    loop {
        if let Some(config) = remote_config.borrow().as_ref() {
            return Some(config.steam_appid);
        }

        if remote_config.changed().await.is_err() {
            return None;
        }
    }
}