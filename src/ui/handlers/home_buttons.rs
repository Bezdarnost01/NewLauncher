use crate::ui::AppWindow;

use i_slint_backend_winit::WinitWindowAccessor;
use slint::ComponentHandle;

pub fn bind_window_drag(app: &AppWindow) {
    let app_weak = app.as_weak();

    app.on_window_drag(move || {
        let Some(app) = app_weak.upgrade() else {
            return;
        };

        app.window().with_winit_window(|window| {
            let _ = window.drag_window();
        });
    });
}

pub fn bind_tools_button(app: &AppWindow) {
    app.on_close_clicked(move || {
        let _ = slint::quit_event_loop();
    });

    let app_weak = app.as_weak();

    app.on_minimize_clicked(move || {
        if let Some(app) = app_weak.upgrade() {
            app.window().set_minimized(true);
        }
    });
}

pub fn bind_links(app: &AppWindow) {
    let app_weak = app.as_weak();

    app.on_open_discord(move || {
        if let Some(app) = app_weak.upgrade() {
            let link = app.get_discord_link();
            let _ = open::that(link.as_str());
        }
    });

    let app_weak = app.as_weak();

    app.on_open_telegram(move || {
        if let Some(app) = app_weak.upgrade() {
            let link = app.get_telegram_link();
            let _ = open::that(link.as_str());
        }
    });
}
