use std::time::Duration;

use crate::ui::AppWindow;

use slint::ComponentHandle;
use tokio::time::{MissedTickBehavior, interval};

use crate::integrations::steam::SteamClient;

const APP_ID: u32 = 22010;
const REFRESH_INTERVAL: Duration = Duration::from_secs(15);

pub fn start(app: &AppWindow, runtime: &tokio::runtime::Runtime) {
    let steam = SteamClient::new();
    let app_weak = app.as_weak();

    runtime.spawn(async move {
        let mut ticker = interval(REFRESH_INTERVAL);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            ticker.tick().await;

            let online_count = steam.current_players(APP_ID).await;

            let result = app_weak.upgrade_in_event_loop(move|app|{
                app.set_online_count(online_count as i32);
            });

            if result.is_err() {
                break;
            }
        }
    });
}