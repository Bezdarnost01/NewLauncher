use crate::{
    ui::{AppWindow},
};

use home_buttons::{bind_tools_button, bind_links, bind_window_drag};

pub mod home_buttons;

pub fn bind(app: &AppWindow) {

    bind_window_drag(app);
    bind_tools_button(app);
    bind_links(app);
}
