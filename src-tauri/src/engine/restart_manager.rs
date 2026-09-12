use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use serde::{Deserialize, Serialize};

use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_MORE_DATA, ERROR_SUCCESS, BOOL, HANDLE, HWND, LPARAM,
};
use windows_sys::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows_sys::Win32::System::RestartManager::{
    RmEndSession, RmGetList, RmRegisterResources, RmStartSession,
    CCH_RM_SESSION_KEY, RM_PROCESS_INFO,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW,
    PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
};

use super::friendly_names::get_friendly_name;
use super::risk::assess_process_risk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub app_name: String,
    pub friendly_name: Option<String>,
    pub exe_path: Option<String>,
    pub window_title: Option<String>,
    pub risk_level: String,
    pub is_service: bool,
}

struct RmSessionGuard(u32);

impl Drop for RmSessionGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = RmEndSession(self.0);
        }
    }
}

/// Поиск всех процессов, удерживающих файл или папку, через Win32 Restart Manager API
pub fn get_locking_processes<P: AsRef<Path>>(target_path: P) -> Result<Vec<ProcessInfo>, String> {
    let path = target_path.as_ref();
    let full_path = std::fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf());

    let path_str = full_path.to_string_lossy();
    // Strip Windows extended prefix \\?\ if present
    let clean_path = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);

    let wide_path: Vec<u16> = clean_path
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut session_handle: u32 = 0;
        let mut session_key = [0u16; CCH_RM_SESSION_KEY as usize + 1];

        let start_res = RmStartSession(&mut session_handle, 0, session_key.as_mut_ptr());
        if start_res != ERROR_SUCCESS {
            return Err(format!("RmStartSession failed with error code: {}", start_res));
        }

        let _guard = RmSessionGuard(session_handle);

        let path_ptrs = [wide_path.as_ptr()];
        let reg_res = RmRegisterResources(
            session_handle,
            1,
            path_ptrs.as_ptr(),
            0,
            std::ptr::null(),
            0,
            std::ptr::null(),
        );

        if reg_res != ERROR_SUCCESS {
            return Err(format!("RmRegisterResources failed with error code: {}", reg_res));
        }

        let mut proc_info_needed: u32 = 0;
        let mut proc_info: u32 = 0;
        let mut reboot_reasons: u32 = 0;

        let get_res = RmGetList(
            session_handle,
            &mut proc_info_needed,
            &mut proc_info,
            std::ptr::null_mut(),
            &mut reboot_reasons,
        );

        if get_res != ERROR_SUCCESS && get_res != ERROR_MORE_DATA {
            return Err(format!("RmGetList size query failed with error code: {}", get_res));
        }

        if proc_info_needed == 0 {
            return Ok(Vec::new());
        }

        let mut affected_apps: Vec<RM_PROCESS_INFO> = vec![std::mem::zeroed(); proc_info_needed as usize];
        proc_info = proc_info_needed;

        let get_list_res = RmGetList(
            session_handle,
            &mut proc_info_needed,
            &mut proc_info,
            affected_apps.as_mut_ptr(),
            &mut reboot_reasons,
        );

        if get_list_res != ERROR_SUCCESS {
            return Err(format!("RmGetList items fetch failed with error code: {}", get_list_res));
        }

        let count = proc_info.min(proc_info_needed) as usize;
        let mut results = Vec::with_capacity(count);

        for app in affected_apps.into_iter().take(count) {
            let pid = app.Process.dwProcessId;

            let app_name = null_terminated_utf16(&app.strAppName);
            let svc_name = null_terminated_utf16(&app.strServiceShortName);
            let is_service = !svc_name.is_empty();

            let exe_path = get_process_exe_path(pid);
            let window_title = get_window_title_for_pid(pid);

            let exe_filename = exe_path
                .as_deref()
                .and_then(|p| Path::new(p).file_name())
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| {
                    if !app_name.is_empty() {
                        app_name.clone()
                    } else {
                        format!("PID_{}", pid)
                    }
                });

            let friendly = get_friendly_name(&exe_filename).map(|s| s.to_string());
            let risk = assess_process_risk(&exe_filename);

            results.push(ProcessInfo {
                pid,
                name: exe_filename,
                app_name: if !app_name.is_empty() { app_name } else { svc_name },
                friendly_name: friendly,
                exe_path,
                window_title,
                risk_level: risk.as_str().to_string(),
                is_service,
            });
        }

        Ok(results)
    }
}

fn null_terminated_utf16(slice: &[u16]) -> String {
    let len = slice.iter().position(|&c| c == 0).unwrap_or(slice.len());
    String::from_utf16_lossy(&slice[..len])
}

fn get_process_exe_path(pid: u32) -> Option<String> {
    unsafe {
        let handle: HANDLE = OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_QUERY_INFORMATION,
            0,
            pid,
        );

        if handle.is_null() {
            // Try limited rights
            let limited_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if limited_handle.is_null() {
                return None;
            }
            return query_image_name(limited_handle);
        }

        let result = query_image_name(handle);
        CloseHandle(handle);
        result
    }
}

unsafe fn query_image_name(handle: HANDLE) -> Option<String> {
    let mut buffer = [0u16; 1024];
    let mut size = buffer.len() as u32;

    if QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut size) != 0 && size > 0 {
        let os_str = OsString::from_wide(&buffer[..size as usize]);
        CloseHandle(handle);
        return Some(os_str.to_string_lossy().to_string());
    }

    // Fallback to GetModuleFileNameExW if QueryFullProcessImageNameW is not available
    let mod_len = GetModuleFileNameExW(handle, std::ptr::null_mut(), buffer.as_mut_ptr(), buffer.len() as u32);
    CloseHandle(handle);

    if mod_len > 0 {
        let os_str = OsString::from_wide(&buffer[..mod_len as usize]);
        Some(os_str.to_string_lossy().to_string())
    } else {
        None
    }
}

struct WindowSearchContext {
    target_pid: u32,
    found_title: Option<String>,
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let ctx = &mut *(lparam as *mut WindowSearchContext);

    let mut win_pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, &mut win_pid);

    if win_pid == ctx.target_pid && IsWindowVisible(hwnd) != 0 {
        let title_len = GetWindowTextLengthW(hwnd);
        if title_len > 0 {
            let mut buf = vec![0u16; (title_len + 1) as usize];
            let read = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
            if read > 0 {
                let title = String::from_utf16_lossy(&buf[..read as usize]);
                if !title.trim().is_empty() {
                    ctx.found_title = Some(title);
                    return 0; // Stop enumeration once found
                }
            }
        }
    }

    1 // Continue enumeration
}

fn get_window_title_for_pid(pid: u32) -> Option<String> {
    let mut ctx = WindowSearchContext {
        target_pid: pid,
        found_title: None,
    };

    unsafe {
        EnumWindows(Some(enum_windows_proc), &mut ctx as *mut _ as LPARAM);
    }

    ctx.found_title
}

/// Принудительное завершение процесса по его PID
pub fn kill_process(pid: u32) -> Result<(), String> {
    use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};

    if pid == 0 || pid == 4 {
        return Err("Нельзя завершать системные процессы Windows (System/Idle)".to_string());
    }

    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            return Err(format!("Не удалось получить дескриптор для завершения процесса PID {}", pid));
        }

        let res = TerminateProcess(handle, 1);
        CloseHandle(handle);

        if res == 0 {
            return Err(format!("Не удалось завершить процесс PID {}", pid));
        }

        Ok(())
    }
}

pub fn unlock_all_processes(pids: &[u32]) -> Result<u32, String> {
    let mut killed = 0;
    for &pid in pids {
        if pid == 0 || pid == 4 {
            continue;
        }
        if kill_process(pid).is_ok() {
            killed += 1;
        }
    }
    Ok(killed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_detect_locking_process_on_open_file() {
        let temp_dir = std::env::temp_dir();
        let test_file_path = temp_dir.join(format!("freeit_test_lock_{}.tmp", std::process::id()));

        // Open and keep file handle open
        let file = File::create(&test_file_path).expect("failed to create temp file");

        let locking = get_locking_processes(&test_file_path).expect("failed to get locking processes");

        let current_pid = std::process::id();
        let found = locking.iter().any(|p| p.pid == current_pid);

        // Drop handle and cleanup
        drop(file);
        let _ = std::fs::remove_file(&test_file_path);

        assert!(
            found,
            "Expected current process PID {} to be detected by Restart Manager, got: {:?}",
            current_pid, locking
        );
    }
}

