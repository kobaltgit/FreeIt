/// Возвращает понятное человекочитаемое имя для известных процессов Windows и приложений
pub fn get_friendly_name(exe_name: &str) -> Option<&'static str> {
    let lower = exe_name.to_lowercase();
    let name = lower.trim_end_matches(".exe");

    let friendly = match name {
        // Разработка и серверы
        "node" => "Node.js (Vite / Next.js / Dev Server)",
        "code" => "Visual Studio Code",
        "devenv" => "Visual Studio IDE",
        "rustc" => "Rust Compiler",
        "cargo" => "Rust Cargo Package Manager",
        "git" => "Git Version Control",
        "python" | "pythonw" => "Python Runtime",
        "java" | "javaw" => "Java Virtual Machine",
        "dotnet" => ".NET Runtime",
        "docker" | "dockerd" => "Docker Desktop / Engine",

        // Системные процессы Windows
        "explorer" => "Проводник Windows (Файловый менеджер)",
        "svchost" => "Хост-процесс служб Windows",
        "searchindexer" => "Служба индексирования поиска Windows",
        "spoolsv" => "Диспетчер очереди печати Windows",
        "dwm" => "Диспетчер окон рабочего стола (DWM)",
        "csrss" => "Процесс исполнения клиент-серверной подсистемы",
        "lsass" => "Локальная служба безопасности Windows",
        "services" => "Диспетчер системных служб Windows",
        "winlogon" => "Программа входа в систему Windows",

        // Синхронизация и облачные диски
        "onedrive" => "Microsoft OneDrive (Облачная синхронизация)",
        "dropbox" => "Dropbox (Облачная синхронизация)",
        "googledrivesync" => "Google Диск (Облачная синхронизация)",
        "yandexdisk" => "Яндекс Диск (Синхронизация)",

        // Браузеры
        "chrome" => "Google Chrome",
        "msedge" => "Microsoft Edge",
        "firefox" => "Mozilla Firefox",
        "brave" => "Brave Browser",
        "opera" => "Opera Browser",

        // Медиа, дизайн и офис
        "photoshop" => "Adobe Photoshop",
        "illustrator" => "Adobe Illustrator",
        "premiere" => "Adobe Premiere Pro",
        "afterfx" => "Adobe After Effects",
        "figma" => "Figma Desktop",
        "winword" => "Microsoft Word",
        // Экосистема Kobalt Tools
        "stashit" => "StashIt (Плавающий буфер-карман у курсора для сбора и перетаскивания файлов/текста)",
        "polyshift" => "PolyShift (Умный переключатель раскладки клавиатуры и ИИ-помощник)",
        "minibin" => "MiniBin (Компактный значок Корзины в системном трее)",
        "peekit" => "PeekIt (Быстрый предпросмотр файлов по нажатию Space)",
        "freeit" => "FreeIt (Разблокировка, переименование и безопасное удаление занятых файлов)",
        "undoit" => "Undoit (Локальная машина времени и версионирование файлов)",

        // Офис, плееры, мессенджеры
        "excel" => "Microsoft Excel",
        "powerpnt" => "Microsoft PowerPoint",
        "telegram" => "Telegram Desktop",
        "discord" => "Discord",
        "spotify" => "Spotify",
        "steam" => "Steam Client",
        _ => return None,
    };

    Some(friendly)
}

#[cfg(target_os = "windows")]
#[link(name = "version")]
extern "system" {}

#[cfg(target_os = "windows")]
pub fn get_pe_file_description(exe_path: &str) -> Option<String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
    };

    let mut wide_path: Vec<u16> = OsStr::new(exe_path).encode_wide().collect();
    wide_path.push(0);

    let mut handle: u32 = 0;
    let size = unsafe { GetFileVersionInfoSizeW(wide_path.as_ptr(), &mut handle) };
    if size == 0 {
        return None;
    }

    let mut buffer = vec![0u8; size as usize];
    let success = unsafe { GetFileVersionInfoW(wide_path.as_ptr(), 0, size, buffer.as_mut_ptr() as _) };
    if success == 0 {
        return None;
    }

    // Ищем языковой блок: \VarFileInfo\Translation
    let sub_block: Vec<u16> = OsStr::new("\\VarFileInfo\\Translation")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut lp_buffer: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut pu_len: u32 = 0;

    let res = unsafe {
        VerQueryValueW(
            buffer.as_ptr() as _,
            sub_block.as_ptr(),
            &mut lp_buffer,
            &mut pu_len,
        )
    };

    let (lang, codepage) = if res != 0 && pu_len >= 4 && !lp_buffer.is_null() {
        let trans = lp_buffer as *const u16;
        let lang = unsafe { *trans };
        let cp = unsafe { *trans.add(1) };
        (lang, cp)
    } else {
        (0x0409, 0x04b0) // en-US Unicode
    };

    // Перебираем ключевые текстовые поля метаданных Windows
    for field in &["FileDescription", "ProductName", "CompanyName"] {
        let query_str = format!("\\StringFileInfo\\{:04x}{:04x}\\{}\0", lang, codepage, field);
        let query_wide: Vec<u16> = query_str.encode_utf16().collect();

        let mut str_buf: *mut std::ffi::c_void = std::ptr::null_mut();
        let mut str_len: u32 = 0;
        let found = unsafe {
            VerQueryValueW(
                buffer.as_ptr() as _,
                query_wide.as_ptr(),
                &mut str_buf,
                &mut str_len,
            )
        };

        if found != 0 && str_len > 1 && !str_buf.is_null() {
            let slice = unsafe { std::slice::from_raw_parts(str_buf as *const u16, str_len as usize) };
            let len = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
            let desc = String::from_utf16_lossy(&slice[..len]);
            let trimmed = desc.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }

    None
}

#[cfg(not(target_os = "windows"))]
pub fn get_pe_file_description(_exe_path: &str) -> Option<String> {
    None
}
