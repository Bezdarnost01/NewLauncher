use crate::{
    config::Config,
    integrations::{
        api::{ApiClient, config_channel},
        steam::SteamClient,
    },
    ui::{AppWindow, backgrounds, handlers, online_status, update_info},
};

pub fn create(
    runtime: &tokio::runtime::Runtime,
    api: ApiClient,
    steam: SteamClient,
    user_config: Config,
) -> Result<AppWindow, slint::PlatformError> {
    let app = AppWindow::new()?;
    let user_config = std::rc::Rc::new(std::cell::RefCell::new(user_config));
    handlers::bind(&app, user_config.clone());

    let (config_tx, config_rx) = config_channel();

    let startup_config = user_config.borrow().clone();
    runtime.spawn(async move {
        let Some(remote) = api.fetch_config().await else {
            log::error!("could not obtain remote config");
            return;
        };

        if let Err(err) = api.download_backgrounds(&startup_config, &remote).await {
            log::warn!("download backgrounds failed: {err}");
        }

        let _ = config_tx.send(Some(remote));
    });

    online_status::start(&app, runtime, steam, config_rx.clone());
    backgrounds::start(&app, runtime, config_rx.clone(), user_config);
    update_info::update(&app, runtime, config_rx);

    Ok(app)
}
