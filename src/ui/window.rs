use crate::{
    integrations::api::{ApiClient, config_channel},
    ui::{AppWindow, handlers, online_status, update_info},
};

pub fn create(runtime: &tokio::runtime::Runtime) -> Result<AppWindow, slint::PlatformError> {
    let app = AppWindow::new()?;

    let api_client = ApiClient::new();

    let (config_sender, remote_config) = config_channel();

    runtime.spawn(async move {
        match api_client.get_config().await {
            Ok(config) => {
                let _ = config_sender.send(Some(config));
            }
            Err(error) => {
                eprintln!("failed to load remote config: {error}");
            }
        }
    });

    handlers::bind(&app);

    online_status::start(&app, runtime, remote_config.clone());
    update_info::update(&app, runtime, remote_config.clone());

    Ok(app)
}