use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowExW, FindWindowW, SendMessageTimeoutW, SMTO_NORMAL,
};
use windows::core::w;

const WM_SPAWN_WORKER: u32 = 0x052C;

pub fn get_wallpaper_worker_window() -> HWND {
    unsafe {
        let progman = FindWindowW(w!("Progman"), None);

        // Send message to create Worker W
        SendMessageTimeoutW(
            progman,
            WM_SPAWN_WORKER,
            WPARAM(0),
            LPARAM(0),
            SMTO_NORMAL,
            1000,
            None,
        );

        thread::sleep(Duration::from_millis(100));

        let mut worker_w = HWND(0);

        // Callback to iterate files
        unsafe extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
            let p_worker_w = lparam.0 as *mut HWND;
            
            let shell_dll = FindWindowExW(window, HWND(0), w!("SHELLDLL_DefView"), None);
            
            if shell_dll.0 != 0 {
                let target_worker = FindWindowExW(HWND(0), window, w!("WorkerW"), None);
                if target_worker.0 != 0 {
                    *p_worker_w = target_worker;
                }
            }
            true.into()
        }

        // Launch search
        match EnumWindows(Some(enum_window), LPARAM(&mut worker_w as *mut _ as isize)) {
            Ok(()) => (),
            Err(e) => eprintln!("Worker W search failed: {e}")
        };

        worker_w
    }
}