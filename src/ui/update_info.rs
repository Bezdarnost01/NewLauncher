use slint::ComponentHandle;

use crate::{
    integrations::api::{ApiConfig, RemoteConfig},
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
        });

        if result.is_err() {
            eprintln!("failed to update app config in UI");
        }
    });
}

async fn wait_config(remote_config: &mut RemoteConfig) -> Option<ApiConfig> {
    loop {
        let config = {
            remote_config.borrow().clone()
        };

        if let Some(config) = config {
            return Some(config);
        }

        if remote_config.changed().await.is_err() {
            return None;
        }
    }
}