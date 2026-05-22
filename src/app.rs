use std::time::Duration;

use slint::ComponentHandle;

use crate::{
    config,
    integrations::{api::ApiClient, http::HttpClient, steam::SteamClient},
    ui,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;

    let http = HttpClient::new()?;
    let api = ApiClient::new(http.clone());
    let steam = SteamClient::new(http);

    let user_config = config::load_config();

    let app = ui::window::create(&runtime, api, steam, user_config)?;

    app.run()?;

    runtime.shutdown_timeout(Duration::from_secs(3));
    Ok(())
}
