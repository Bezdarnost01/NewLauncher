use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use slint::{ComponentHandle, Image, Model, ModelRc, SharedString, Timer, TimerMode, VecModel};

use crate::{
    config::{backgrounds_dir, save_config},
    integrations::api::{wait_config, ApiBackground, RemoteConfig},
    ui::{AppWindow, BackgroundItem, SettingsState, SharedConfig},
};

#[derive(Clone)]
struct BackgroundEntry {
    name: String,
    path: PathBuf,
}

thread_local! {
    static RANDOM_BACKGROUND_TIMER: RefCell<Option<Timer>> = RefCell::new(None);
}

pub fn start(
    app: &AppWindow,
    runtime: &tokio::runtime::Runtime,
    mut remote_config: RemoteConfig,
    config: SharedConfig,
) {
    let initial_config = config.borrow().clone();
    let app_weak = app.as_weak();
    start_random_timer(app, config.clone());

    runtime.spawn(async move {
        let Some(remote) = wait_config(&mut remote_config).await else {
            return;
        };

        let entries = collect_backgrounds(&remote.backgrounds);
        if entries.is_empty() {
            log::warn!("no backgrounds available");
            return;
        }

        let selected_name = selected_or_first(&entries, &initial_config.background_name);
        let background_enabled = initial_config.background_enabled;

        let result = app_weak.upgrade_in_event_loop(move |app| {
            app.set_backgrounds(background_model(&entries));
            app.set_selected_background_name(selected_name.clone().into());
            app.set_use_background_image(background_enabled);

            if let Some(entry) = find_background(&entries, &selected_name) {
                set_background_image(&app, entry);
            }
        });

        if result.is_err() {
            log::warn!("failed to update backgrounds in UI");
        }
    });

    let app_weak = app.as_weak();
    app.on_background_selected(move |name| {
        let name = name.to_string();

        let Some(app) = app_weak.upgrade() else {
            return;
        };

        let Some(item) = find_background_item(&app, &name) else {
            log::warn!("selected background not found: {name}");
            return;
        };

        update_config(&config, |cfg| {
            cfg.background_name = name.clone();
            cfg.background_enabled = true;
        });

        app.global::<SettingsState>().set_background_enabled(true);
        app.set_selected_background_name(item.name);
        app.set_background_image(item.image);
        app.set_use_background_image(true);
    });
}

fn start_random_timer(app: &AppWindow, config: SharedConfig) {
    let app_weak = app.as_weak();

    RANDOM_BACKGROUND_TIMER.with(|slot| {
        let timer = Timer::default();

        timer.start(TimerMode::Repeated, Duration::from_secs(30), move || {
            let config_snapshot = config.borrow().clone();
            if !config_snapshot.background_enabled || !config_snapshot.random_backgrounds_enabled {
                return;
            }

            let Some(app) = app_weak.upgrade() else {
                return;
            };

            select_next_background(&app, &config);
        });

        *slot.borrow_mut() = Some(timer);
    });
}

fn select_next_background(app: &AppWindow, config: &SharedConfig) {
    let backgrounds = app.get_backgrounds();
    let row_count = backgrounds.row_count();

    if row_count < 2 {
        return;
    }

    let selected_name = app.get_selected_background_name().to_string();
    let mut selected_index = None;

    for index in 0..row_count {
        let Some(item) = backgrounds.row_data(index) else {
            continue;
        };

        if item.name.as_str() == selected_name {
            selected_index = Some(index);
            break;
        }
    }

    let next_index = random_background_index(row_count, selected_index);
    let Some(next) = backgrounds.row_data(next_index) else {
        return;
    };

    let next_name = next.name.to_string();
    app.set_background_image(next.image.clone());
    app.set_selected_background_name(next.name);
    app.set_use_background_image(true);

    update_config(config, |cfg| cfg.background_name = next_name);
}

fn random_background_index(row_count: usize, selected_index: Option<usize>) -> usize {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.subsec_nanos() as usize)
        .unwrap_or_default();

    let index = nanos % row_count;

    match selected_index {
        Some(current) if row_count > 1 && index == current => (index + 1) % row_count,
        _ => index,
    }
}

fn collect_backgrounds(backgrounds: &[ApiBackground]) -> Vec<BackgroundEntry> {
    backgrounds
        .iter()
        .filter_map(|background| {
            resolve_background_path(background).map(|path| BackgroundEntry {
                name: background.name.clone(),
                path,
            })
        })
        .collect()
}

fn resolve_background_path(background: &ApiBackground) -> Option<PathBuf> {
    let downloaded = backgrounds_dir().join(&background.file);
    if downloaded.exists() {
        return Some(downloaded);
    }

    let bundled = PathBuf::from("assets")
        .join("remote")
        .join("backgrounds")
        .join(&background.file);

    bundled.exists().then_some(bundled)
}

fn selected_or_first(entries: &[BackgroundEntry], selected_name: &str) -> String {
    entries
        .iter()
        .find(|entry| entry.name == selected_name)
        .or_else(|| entries.first())
        .map(|entry| entry.name.clone())
        .unwrap_or_default()
}

fn find_background<'a>(
    entries: &'a [BackgroundEntry],
    selected_name: &str,
) -> Option<&'a BackgroundEntry> {
    entries.iter().find(|entry| entry.name == selected_name)
}

fn find_background_item(app: &AppWindow, selected_name: &str) -> Option<BackgroundItem> {
    let backgrounds = app.get_backgrounds();

    for index in 0..backgrounds.row_count() {
        let item = backgrounds.row_data(index)?;

        if item.name.as_str() == selected_name {
            return Some(item);
        }
    }

    None
}

fn background_model(entries: &[BackgroundEntry]) -> ModelRc<BackgroundItem> {
    let items = entries
        .iter()
        .map(|entry| BackgroundItem {
            name: SharedString::from(entry.name.as_str()),
            image: load_image(&entry.path),
        })
        .collect::<Vec<_>>();

    ModelRc::from(Rc::new(VecModel::from(items)))
}

fn set_background_image(app: &AppWindow, entry: &BackgroundEntry) {
    app.set_background_image(load_image(&entry.path));
}

fn load_image(path: &Path) -> Image {
    Image::load_from_path(path).unwrap_or_else(|err| {
        log::warn!("failed to load background image {}: {err}", path.display());
        Image::default()
    })
}

fn update_config(config: &SharedConfig, update: impl FnOnce(&mut crate::config::Config)) {
    let mut config = config.borrow_mut();
    update(&mut config);

    if let Err(err) = save_config(&config) {
        log::warn!("failed to save user config: {err}");
    }
}
