use crate::{config::Config, ui::AppWindow};

use home_buttons::{bind_tools_button, bind_links, bind_window_drag};
use settings::bind_settings;

pub mod home_buttons;
pub mod settings;

pub fn bind(app: &AppWindow, user_config: &Config) {

    bind_window_drag(app);
    bind_tools_button(app);
    bind_links(app);
    bind_settings(app, user_config);
}
