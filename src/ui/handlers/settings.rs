use std::{cell::RefCell, rc::Rc};

use slint::ComponentHandle;

use crate::{
    config::{save_config, Config},
    ui::{AppWindow, SettingsState},
};

pub fn bind_settings(app: &AppWindow, user_config: &Config) {
    let settings = app.global::<SettingsState>();

    settings.set_discord_rpc_enabled(user_config.discord_rpc_enabled);
    settings.set_anti_cheat_enabled(user_config.anti_cheat_enabled);
    settings.set_match_alert_enabled(user_config.match_alert_enabled);
    settings.set_background_enabled(user_config.background_enabled);
    settings.set_random_backgrounds_enabled(user_config.random_backgrounds_enabled);
    app.set_game_path(user_config.game_folder.clone().into());

    let config = Rc::new(RefCell::new(user_config.clone()));

    {
        let config = Rc::clone(&config);
        settings.on_discord_rpc_toggled(move |value| {
            update_config(&config, |cfg| cfg.discord_rpc_enabled = value);
        });
    }

    {
        let config = Rc::clone(&config);
        settings.on_anti_cheat_toggled(move |value| {
            update_config(&config, |cfg| cfg.anti_cheat_enabled = value);
        });
    }

    {
        let config = Rc::clone(&config);
        settings.on_match_alert_toggled(move |value| {
            update_config(&config, |cfg| cfg.match_alert_enabled = value);
        });
    }

    {
        let config = Rc::clone(&config);
        settings.on_background_toggled(move |value| {
            update_config(&config, |cfg| cfg.background_enabled = value);
        });
    }

    {
        let config = Rc::clone(&config);
        settings.on_random_backgrounds_toggled(move |value| {
            update_config(&config, |cfg| cfg.random_backgrounds_enabled = value);
        });
    }

    let app_weak = app.as_weak();
    app.on_open_change_path(move || {
        let Some(folder) = rfd::FileDialog::new().pick_folder() else {
            return;
        };

        let game_folder = folder.to_string_lossy().into_owned();
        update_config(&config, |cfg| cfg.game_folder = game_folder.clone());

        if let Some(app) = app_weak.upgrade() {
            app.set_game_path(game_folder.into());
        }
    });
}

fn update_config(config: &Rc<RefCell<Config>>, update: impl FnOnce(&mut Config)) {
    let mut config = config.borrow_mut();
    update(&mut config);

    if let Err(err) = save_config(&config) {
        log::warn!("failed to save user config: {err}");
    }
}
