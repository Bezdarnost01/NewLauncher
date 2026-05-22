use crate::{
    config::Config,
    integrations::{
        api::{ApiClient, config_channel},
        steam::SteamClient,
    },
    ui::{AppWindow, handlers, online_status, update_info},
};

pub fn create(
    runtime: &tokio::runtime::Runtime,
    api: ApiClient,
    steam: SteamClient,
    user_config: Config,
) -> Result<AppWindow, slint::PlatformError> {
    let app = AppWindow::new()?;
    handlers::bind(&app, &user_config);

    let (config_tx, config_rx) = config_channel();

    runtime.spawn(async move {
        let Some(remote) = api.fetch_config().await else {
            log::error!("could not obtain remote config");
            return;
        };

        if let Err(err) = api.download_backgrounds(&user_config, &remote).await {
            log::warn!("download backgrounds failed: {err}");
        }

        let _ = config_tx.send(Some(remote));
    });

    online_status::start(&app, runtime, steam, config_rx.clone());
    update_info::update(&app, runtime, config_rx);

    Ok(app)
}
