use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub gemini_api_key: String,
    pub gemini_model: String,
    pub auto_start: bool,
    pub safe_vault: bool,
    pub tray_icon_style: String,
    pub explorer_context_menu: bool,
    pub language: String,
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            gemini_api_key: String::new(),
            gemini_model: "gemini-2.5-flash".to_string(),
            auto_start: false,
            safe_vault: true,
            tray_icon_style: "white".to_string(),
            explorer_context_menu: true,
            language: "ru".to_string(),
            theme: "dark".to_string(),
        }
    }
}

pub fn get_settings_dir() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    let dir = PathBuf::from(appdata).join("Kobalt").join("FreeIt");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn get_settings_file_path() -> PathBuf {
    get_settings_dir().join("settings.json")
}

pub fn load_settings() -> AppSettings {
    let path = get_settings_file_path();
    let mut settings = if let Ok(content) = fs::read_to_string(&path) {
        serde_json::from_str::<AppSettings>(&content).unwrap_or_default()
    } else {
        AppSettings::default()
    };

    // Если ключ пустой, пробуем подтянуть общий ключ экосистемы Kobalt из реестра (как в PolyShift)
    if settings.gemini_api_key.trim().is_empty() {
        if let Some(shared_key) = read_registry_string(r"Software\KobaltTools\API", "GeminiKey") {
            settings.gemini_api_key = shared_key;
        }
    }
    if settings.gemini_model.trim().is_empty() {
        if let Some(shared_model) = read_registry_string(r"Software\KobaltTools\API", "GeminiModel") {
            settings.gemini_model = shared_model;
        }
    }

    settings
}

pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let path = get_settings_file_path();
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Ошибка сериализации настроек: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Ошибка записи файла настроек: {}", e))?;

    // Синхронизация общего ключа Kobalt в реестре (как в PolyShift)
    if !settings.gemini_api_key.trim().is_empty() {
        let _ = write_registry_string(r"Software\KobaltTools\API", "GeminiKey", &settings.gemini_api_key);
        let _ = write_registry_string(r"Software\KobaltTools\API", "GeminiModel", &settings.gemini_model);
    }

    // Синхронизация автозапуска
    let _ = sync_autostart_registry(settings.auto_start);

    // Синхронизация контекстного меню Проводника
    let _ = sync_context_menu_registry(settings.explorer_context_menu);

    Ok(())
}

fn sync_autostart_registry(enable: bool) -> Result<(), String> {
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Не удалось получить путь к текущему exe: {}", e))?;
    let exe_str = format!("\"{}\"", current_exe.to_string_lossy());

    if enable {
        write_registry_string(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "FreeIt",
            &exe_str,
        )?;
    } else {
        delete_registry_value(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "FreeIt",
        )?;
    }
    Ok(())
}

/// Регистрация/удаление пункта «Разблокировать в FreeIt» в контекстном меню Проводника Windows (для файлов, папок и дисков)
pub fn sync_context_menu_registry(enable: bool) -> Result<(), String> {
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Не удалось получить путь к текущему exe: {}", e))?;
    let exe_path = current_exe.to_string_lossy().to_string();
    let cmd_str = format!("\"{}\" \"%1\"", exe_path);
    let icon_str = format!("\"{}\"", exe_path);

    let targets = [
        r"Software\Classes\*\shell\FreeIt",
        r"Software\Classes\Directory\shell\FreeIt",
        r"Software\Classes\Drive\shell\FreeIt",
    ];

    for target in targets {
        if enable {
            let _ = write_registry_string(target, "", "Разблокировать в FreeIt");
            let _ = write_registry_string(target, "Icon", &icon_str);
            let cmd_sub = format!(r"{}\command", target);
            let _ = write_registry_string(&cmd_sub, "", &cmd_str);
        } else {
            let cmd_sub = format!(r"{}\command", target);
            let _ = delete_registry_key(&cmd_sub);
            let _ = delete_registry_key(target);
        }
    }

    Ok(())
}

// --- Безопасные Win32 Registry утилиты ---

#[cfg(target_os = "windows")]
fn read_registry_string(sub_key: &str, value_name: &str) -> Option<String> {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER, KEY_READ, REG_SZ,
    };

    let wide_sub_key: Vec<u16> = sub_key.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_val_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut hkey: HKEY = std::ptr::null_mut();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            wide_sub_key.as_ptr(),
            0,
            KEY_READ,
            &mut hkey,
        ) != 0
        {
            return None;
        }

        let mut val_type: u32 = 0;
        let mut byte_size: u32 = 0;

        let query_res = RegQueryValueExW(
            hkey,
            wide_val_name.as_ptr(),
            std::ptr::null_mut(),
            &mut val_type,
            std::ptr::null_mut(),
            &mut byte_size,
        );

        if query_res != 0 || val_type != REG_SZ || byte_size == 0 {
            RegCloseKey(hkey);
            return None;
        }

        let mut buf = vec![0u8; byte_size as usize];
        let read_res = RegQueryValueExW(
            hkey,
            wide_val_name.as_ptr(),
            std::ptr::null_mut(),
            &mut val_type,
            buf.as_mut_ptr(),
            &mut byte_size,
        );

        RegCloseKey(hkey);

        if read_res == 0 {
            let u16_slice = std::slice::from_raw_parts(
                buf.as_ptr() as *const u16,
                (byte_size as usize) / 2,
            );
            let mut s = String::from_utf16_lossy(u16_slice);
            if let Some(pos) = s.find('\0') {
                s.truncate(pos);
            }
            Some(s)
        } else {
            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn read_registry_string(_sub_key: &str, _value_name: &str) -> Option<String> {
    None
}

#[cfg(target_os = "windows")]
fn write_registry_string(sub_key: &str, value_name: &str, value: &str) -> Result<(), String> {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegCreateKeyW, RegSetValueExW, HKEY, HKEY_CURRENT_USER, REG_SZ,
    };

    let wide_sub_key: Vec<u16> = sub_key.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_val_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_val_data: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut hkey: HKEY = std::ptr::null_mut();
        let create_res = RegCreateKeyW(
            HKEY_CURRENT_USER,
            wide_sub_key.as_ptr(),
            &mut hkey,
        );

        if create_res != 0 {
            return Err(format!("Ошибка RegCreateKeyW: {}", create_res));
        }

        let set_res = RegSetValueExW(
            hkey,
            wide_val_name.as_ptr(),
            0,
            REG_SZ,
            wide_val_data.as_ptr() as *const u8,
            (wide_val_data.len() * 2) as u32,
        );

        RegCloseKey(hkey);

        if set_res != 0 {
            return Err(format!("Ошибка RegSetValueExW: {}", set_res));
        }

        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn write_registry_string(_sub_key: &str, _value_name: &str, _value: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn delete_registry_value(sub_key: &str, value_name: &str) -> Result<(), String> {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegDeleteValueW, RegOpenKeyExW, HKEY, HKEY_CURRENT_USER, KEY_WRITE,
    };

    let wide_sub_key: Vec<u16> = sub_key.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_val_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut hkey: HKEY = std::ptr::null_mut();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            wide_sub_key.as_ptr(),
            0,
            KEY_WRITE,
            &mut hkey,
        ) != 0
        {
            return Ok(());
        }

        let _ = RegDeleteValueW(hkey, wide_val_name.as_ptr());
        RegCloseKey(hkey);
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn delete_registry_value(_sub_key: &str, _value_name: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn delete_registry_key(sub_key: &str) -> Result<(), String> {
    use windows_sys::Win32::System::Registry::{
        RegDeleteKeyW, HKEY_CURRENT_USER,
    };

    let wide_sub_key: Vec<u16> = sub_key.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let _ = RegDeleteKeyW(HKEY_CURRENT_USER, wide_sub_key.as_ptr());
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn delete_registry_key(_sub_key: &str) -> Result<(), String> {
    Ok(())
}
