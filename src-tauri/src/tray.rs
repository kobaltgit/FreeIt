use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};

use crate::engine::settings::{load_settings, AppSettings};

pub fn load_tray_icon(settings: &AppSettings) -> Image<'static> {
    let bytes: &'static [u8] = if settings.tray_icon_style == "cyan" {
        include_bytes!("../icons/tray-cyan-32.png")
    } else {
        include_bytes!("../icons/tray-white-32.png")
    };
    Image::from_bytes(bytes).expect("Не удалось загрузить иконку для трея FreeIt")
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

pub fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(visible) = window.is_visible() {
            if visible {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }
    }
}

pub fn create_tray(app: &AppHandle) -> Result<TrayIcon, tauri::Error> {
    let settings = load_settings();

    let title_item = MenuItem::with_id(
        app,
        "title",
        "FreeIt v1.0.0",
        false,
        None::<&str>,
    )?;

    let open_item = MenuItem::with_id(
        app,
        "open",
        "🔍 Открыть FreeIt",
        true,
        None::<&str>,
    )?;

    let pick_file_item = MenuItem::with_id(
        app,
        "pick_file",
        "📄 Выбрать файл...",
        true,
        None::<&str>,
    )?;

    let pick_folder_item = MenuItem::with_id(
        app,
        "pick_folder",
        "📁 Выбрать папку...",
        true,
        None::<&str>,
    )?;

    let settings_item = MenuItem::with_id(
        app,
        "settings",
        "⚙️ Настройки",
        true,
        None::<&str>,
    )?;

    let quit_item = MenuItem::with_id(
        app,
        "quit",
        "❌ Выход",
        true,
        None::<&str>,
    )?;

    let sep1 = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let sep3 = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &title_item,
            &sep1,
            &open_item,
            &pick_file_item,
            &pick_folder_item,
            &sep2,
            &settings_item,
            &sep3,
            &quit_item,
        ],
    )?;

    let icon = load_tray_icon(&settings);

    let tray = TrayIconBuilder::with_id("main_tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("FreeIt — Разблокировка файлов (Kobalt Tools)")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                show_main_window(app);
            }
            "pick_file" => {
                let app_handle = app.clone();
                std::thread::spawn(move || {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("FreeIt — Выберите заблокированный файл")
                        .pick_file()
                    {
                        let path_str = path.to_string_lossy().to_string();
                        show_main_window(&app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.emit("inspect-path", path_str);
                            let _ = window.emit("switch-tab", "inspect");
                        }
                    }
                });
            }
            "pick_folder" => {
                let app_handle = app.clone();
                std::thread::spawn(move || {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("FreeIt — Выберите заблокированную папку")
                        .pick_folder()
                    {
                        let path_str = path.to_string_lossy().to_string();
                        show_main_window(&app_handle);
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.emit("inspect-path", path_str);
                            let _ = window.emit("switch-tab", "inspect");
                        }
                    }
                });
            }
            "settings" => {
                show_main_window(app);
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("switch-tab", "settings");
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(tray)
}
