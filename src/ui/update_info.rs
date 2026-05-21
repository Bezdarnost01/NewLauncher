use slint::ComponentHandle;

use crate::{
    integrations::api::{RemoteConfig, wait_config},
    ui::AppWindow,
};

pub fn update(
    app: &AppWindow,
    runtime: &tokio::runtime::Runtime,
    mut remote_config: RemoteConfig,
) {
    let app_weak = app.as_weak();

    runtime.spawn(async move {
        let Some(config) = wait_config(&mut remote_config).await else {
            return;
        };

        let version = config.version;
        let last_update_date = config.last_update_date;

        let result = app_weak.upgrade_in_event_loop(move |app| {
            app.set_version(version.into());
            app.set_update_date(last_update_date.into());
            app.set_discord_link(config.discord_link.into());
            app.set_telegram_link(config.telegram_link.into());
        });

        if result.is_err() {
            eprintln!("failed to update app config in UI");
        }
    });
}