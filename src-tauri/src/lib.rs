pub mod engine;
pub mod single_instance;
pub mod tray;

use tauri::{Emitter, Manager, WebviewWindow};

#[tauri::command]
fn get_locking_processes(path: String) -> Result<Vec<engine::ProcessInfo>, String> {
    engine::get_locking_processes(path)
}

#[tauri::command]
fn kill_process(pid: u32) -> Result<(), String> {
    engine::kill_process(pid)
}

#[tauri::command]
fn get_settings() -> Result<engine::AppSettings, String> {
    Ok(engine::load_settings())
}

#[tauri::command]
fn save_settings(settings: engine::AppSettings, app: tauri::AppHandle) -> Result<(), String> {
    engine::save_settings(&settings)?;

    // Обновляем иконку в трее, если изменился стиль
    if let Some(tray_icon) = app.tray_by_id("main_tray") {
        let img = tray::load_tray_icon(&settings);
        let _ = tray_icon.set_icon(Some(img));
    }

    Ok(())
}

#[tauri::command]
async fn ping_gemini(api_key: String) -> engine::PingResult {
    engine::ping_gemini(api_key).await
}

#[tauri::command]
async fn fetch_models(api_key: String) -> Result<Vec<engine::ModelInfo>, String> {
    engine::list_models(&api_key).await
}

#[tauri::command]
fn pick_file_dialog() -> Option<String> {
    rfd::FileDialog::new()
        .set_title("FreeIt — Выберите заблокированный файл")
        .pick_file()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn pick_folder_dialog() -> Option<String> {
    rfd::FileDialog::new()
        .set_title("FreeIt — Выберите заблокированную папку")
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn hide_window(window: WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
fn minimize_window(window: WebviewWindow) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
async fn explain_target(path: String, processes: Vec<engine::ProcessInfo>, lang: Option<String>) -> Result<engine::AiExplanation, String> {
    let settings = engine::load_settings();
    let language = lang.unwrap_or(settings.language);
    engine::explain_file_and_lock(&settings.gemini_api_key, &settings.gemini_model, &language, &path, &processes).await
}

#[tauri::command]
fn delete_to_trash(path: String) -> Result<(), String> {
    engine::delete_to_recycle_bin(&path)
}

#[tauri::command]
fn rename_target(old_path: String, new_name: String) -> Result<String, String> {
    engine::rename_target(&old_path, &new_name)
}

#[tauri::command]
fn unlock_all(pids: Vec<u32>) -> Result<u32, String> {
    engine::unlock_all_processes(&pids)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_locking_processes,
            kill_process,
            unlock_all,
            get_settings,
            save_settings,
            ping_gemini,
            fetch_models,
            pick_file_dialog,
            pick_folder_dialog,
            hide_window,
            minimize_window,
            explain_target,
            delete_to_trash,
            rename_target
        ])
        .setup(|app| {
            // Запуск IPC слушателя для перехвата вызовов из контекстного меню Проводника
            single_instance::start_primary_ipc_listener(app.handle().clone());

            // Создание системного трея (по образцу minibin_new и PolyShift)
            tray::create_tray(&app.handle())?;

            // Разрешение OLE Drag & Drop через UIPI для работы перетаскивания из Проводника даже под админом
            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(hwnd) = window.hwnd() {
                    use windows_sys::Win32::UI::WindowsAndMessaging::{
                        ChangeWindowMessageFilterEx, MSGFLT_ALLOW, WM_COPYDATA, WM_DROPFILES,
                    };
                    unsafe {
                        ChangeWindowMessageFilterEx(
                            hwnd.0 as _,
                            WM_DROPFILES,
                            MSGFLT_ALLOW,
                            std::ptr::null_mut(),
                        );
                        ChangeWindowMessageFilterEx(
                            hwnd.0 as _,
                            WM_COPYDATA,
                            MSGFLT_ALLOW,
                            std::ptr::null_mut(),
                        );
                        ChangeWindowMessageFilterEx(
                            hwnd.0 as _,
                            0x0049, /* WM_COPYGLOBALDATA */
                            MSGFLT_ALLOW,
                            std::ptr::null_mut(),
                        );
                    }
                }
            }

            // Проверка холодного старта с аргументом пути (например, если программа была вызвана из контекстного меню)
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 && !args[1].starts_with("--") {
                let init_path = args[1].clone();
                if let Some(window) = app.get_webview_window("main") {
                    let win_handle = window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(300));
                        let _ = win_handle.emit("inspect-path", init_path);
                        let _ = win_handle.show();
                        let _ = win_handle.unminimize();
                        let _ = win_handle.set_focus();
                    });
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            // При нажатии кнопки закрытия окно не завершает процесс, а скрывается в системный трей
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
