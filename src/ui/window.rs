use crate::{
    ui::{handlers, AppWindow, online_status},
};

pub fn create(runtime: &tokio::runtime::Runtime) -> Result<AppWindow, slint::PlatformError> {
    let app = AppWindow::new()?;

    handlers::bind(&app);

    online_status::start(&app, runtime);

    Ok(app)
}
