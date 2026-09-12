use std::path::Path;

#[cfg(target_os = "windows")]
pub fn delete_to_recycle_bin(path: &str) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::UI::Shell::{
        SHChangeNotify, SHFileOperationW, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FOF_NOERRORUI,
        FOF_SILENT, FO_DELETE, SHCNE_DELETE, SHCNF_PATHW, SHFILEOPSTRUCTW,
    };

    if !Path::new(path).exists() {
        return Err(format!("Файл или папка не существует: {}", path));
    }

    // Windows SHFileOperationW требует строку, оканчивающуюся двумя нулями (double null-terminated)
    let os_str: &OsStr = OsStr::new(path);
    let mut wide: Vec<u16> = os_str.encode_wide().collect();
    wide.push(0);
    wide.push(0);

    let mut op = SHFILEOPSTRUCTW {
        hwnd: std::ptr::null_mut(),
        wFunc: FO_DELETE,
        pFrom: wide.as_ptr(),
        pTo: std::ptr::null(),
        fFlags: (FOF_ALLOWUNDO | FOF_NOCONFIRMATION | FOF_SILENT | FOF_NOERRORUI) as u16,
        fAnyOperationsAborted: 0,
        hNameMappings: std::ptr::null_mut(),
        lpszProgressTitle: std::ptr::null(),
    };

    let result = unsafe { SHFileOperationW(&mut op) };

    if result != 0 || op.fAnyOperationsAborted != 0 {
        return Err(format!("Ошибка удаления в корзину (код ошибки Win32: {})", result));
    }

    // Оповещаем Windows Shell и MiniBin об удалении
    unsafe {
        SHChangeNotify(SHCNE_DELETE as i32, SHCNF_PATHW, wide.as_ptr() as _, std::ptr::null());
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn rename_target(old_path: &str, new_name: &str) -> Result<String, String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::UI::Shell::{SHChangeNotify, SHCNE_RENAMEITEM, SHCNF_PATHW};

    let p = Path::new(old_path);
    if !p.exists() {
        return Err(format!("Файл или папка не существует: {}", old_path));
    }

    let parent = p.parent().ok_or_else(|| "Не удалось определить родительский каталог".to_string())?;
    let new_path = parent.join(new_name);

    if new_path.exists() {
        return Err(format!("Объект с именем «{}» уже существует в этой папке", new_name));
    }

    std::fs::rename(p, &new_path).map_err(|e| format!("Ошибка переименования: {}", e))?;

    // Оповещаем оболочку Windows об изменении имени
    let mut wide_old: Vec<u16> = OsStr::new(old_path).encode_wide().collect();
    wide_old.push(0);
    let mut wide_new: Vec<u16> = OsStr::new(new_path.as_os_str()).encode_wide().collect();
    wide_new.push(0);

    unsafe {
        SHChangeNotify(
            SHCNE_RENAMEITEM as i32,
            SHCNF_PATHW,
            wide_old.as_ptr() as _,
            wide_new.as_ptr() as _,
        );
    }

    Ok(new_path.to_string_lossy().to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn delete_to_recycle_bin(_path: &str) -> Result<(), String> {
    Err("Поддерживается только на Windows".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn rename_target(_old_path: &str, _new_name: &str) -> Result<String, String> {
    Err("Поддерживается только на Windows".to_string())
}
