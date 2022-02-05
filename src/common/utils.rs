use std::ffi::OsStr;
use std::fs;
use std::io::stdin;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::process;
use std::ptr::null_mut;
use std::{thread, time};
use std::env::{current_exe, args};
use log::error;
use winapi::shared::windef::{HWND, POINT as WinPoint, RECT as WinRect};
use winapi::um::winuser::{
    ClientToScreen, FindWindowW, GetAsyncKeyState, GetClientRect, VK_RBUTTON,
};

use crate::common::PixelRect;
use winapi::shared::minwindef::BOOL;
// use winapi::um::processenv::GetCommandLineW;
use winapi::um::processthreadsapi::{ExitProcess, GetCurrentProcessId};
use winapi::um::securitybaseapi::{AllocateAndInitializeSid, CheckTokenMembership, FreeSid};
use winapi::um::shellapi::{ShellExecuteW};
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winnt::{
    DOMAIN_ALIAS_RID_ADMINS, PSID, SECURITY_BUILTIN_DOMAIN_RID, SECURITY_NT_AUTHORITY,
    SID_IDENTIFIER_AUTHORITY,
};
use winapi::um::winuser::GetWindowThreadProcessId;
// use winapi::um::winuser::MessageBoxW;

pub fn encode_wide(s: String) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(&s).encode_wide().chain(once(0)).collect();
    wide
}

pub fn find_window(title: String) -> Result<HWND, String> {
    let wide = encode_wide(title);
    let result: HWND = unsafe { FindWindowW(null_mut(), wide.as_ptr()) };
    if result.is_null() {
        Err(String::from("cannot find window"))
    } else {
        Ok(result)
    }
}

unsafe fn get_client_rect_unsafe(hwnd: HWND) -> Result<PixelRect, String> {
    let mut rect: WinRect = WinRect {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    GetClientRect(hwnd, &mut rect);
    let width: i32 = rect.right;
    let height: i32 = rect.bottom;

    let mut point: WinPoint = WinPoint { x: 0, y: 0 };
    ClientToScreen(hwnd, &mut point as *mut WinPoint);
    let left: i32 = point.x;
    let top: i32 = point.y;

    Ok(PixelRect {
        left,
        top,
        width,
        height,
    })
}

pub fn get_client_rect(hwnd: HWND) -> Result<PixelRect, String> {
    unsafe { get_client_rect_unsafe(hwnd) }
}

pub fn sleep(ms: u32) {
    let time = time::Duration::from_millis(ms as u64);
    thread::sleep(time);
}

pub fn read_file_to_string(path: String) -> String {
    let content = fs::read_to_string(path).unwrap();
    content
}

pub fn error_and_quit(msg: &str) -> ! {
    error!("{}, 按Enter退出", msg);
    let mut s: String = String::new();
    stdin().read_line(&mut s).unwrap();
    process::exit(0);
}

unsafe fn is_admin_unsafe() -> bool {
    let mut authority: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY {
        Value: SECURITY_NT_AUTHORITY,
    };
    let mut group: PSID = null_mut();
    let mut b = AllocateAndInitializeSid(
        &mut authority as *mut SID_IDENTIFIER_AUTHORITY,
        2,
        SECURITY_BUILTIN_DOMAIN_RID,
        DOMAIN_ALIAS_RID_ADMINS,
        0,
        0,
        0,
        0,
        0,
        0,
        &mut group as *mut PSID,
    );
    if b != 0 {
        let r = CheckTokenMembership(null_mut(), group, &mut b as *mut BOOL);
        if r == 0 {
            b = 0;
        }
        FreeSid(group);
    }

    b != 0
}

pub fn is_admin() -> bool {
    unsafe { is_admin_unsafe() }
}

pub fn is_rmb_down() -> bool {
    unsafe {
        let state = GetAsyncKeyState(VK_RBUTTON);
        if state == 0 {
            return false;
        }

        state & 1 > 0
    }
}

pub fn is_console() -> bool {
    unsafe {
        let h = GetConsoleWindow();
        let mut processid: u32 = 0;
        GetWindowThreadProcessId(h, &mut processid);
        if GetCurrentProcessId() == processid {
            true
        } else {
            false
        }
    }
}

pub fn run_as_admin_exit() {
    let mut argv: Vec<String> = args().collect();
    argv.remove(0);

    unsafe {

        ShellExecuteW(
            null_mut(),
            encode_wide("runas".to_string()).as_ptr(),
            encode_wide(current_exe().unwrap().to_str().unwrap().to_string()).as_ptr(),
            encode_wide(argv.join(" ")).as_ptr(),
            null_mut(),
            5,
        );
        ExitProcess(1)
    }
}
