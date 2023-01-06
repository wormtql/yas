use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::ptr::null_mut;
use std::{thread, time};
use std::fs;
use std::io::stdin;
use std::mem::transmute;
use std::process;
use std::time::Duration;

use log::{error, info, warn};
use reqwest::blocking::Client;
use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use winapi::um::winuser::{FindWindowW, GetClientRect, ClientToScreen, GetAsyncKeyState, VK_RBUTTON, SetProcessDPIAware, ShowWindow, SW_RESTORE, SetForegroundWindow, FindWindowExW, GetWindowLongPtrW, GWL_EXSTYLE, GWL_STYLE};
use winapi::shared::windef::{HWND, RECT as WinRect, POINT as WinPoint};
use crate::common::PixelRect;
use winapi::um::winnt::{SID_IDENTIFIER_AUTHORITY, SECURITY_NT_AUTHORITY, PSID, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_ADMINS, CHAR};
use winapi::um::securitybaseapi::{AllocateAndInitializeSid, CheckTokenMembership, FreeSid};
use winapi::shared::minwindef::{BOOL, HINSTANCE};
use crate::dto::GithubTag;
use os_info;
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA, LoadLibraryW};
// use winapi::um::shellscalingapi::{PROCESS_PER_MONITOR_DPI_AWARE, SetProcessDpiAwareness};

pub fn encode_wide(s: String) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(&s).encode_wide().chain(once(0)).collect();
    wide
}

/*
pub fn find_window(title: &str) -> Result<HWND, String> {
    let wide = encode_wide(String::from(title));
    let result: HWND = unsafe {
        FindWindowW(null_mut(), wide.as_ptr())
    };
    if result.is_null() {
        Err(String::from("cannot find window"))
    } else {
        Ok(result)
    }
}
*/

pub fn find_window_local() -> Result<HWND, String> {
    let title = encode_wide(String::from("原神"));
    let class = encode_wide(String::from("UnityWndClass"));
    let result: HWND = unsafe {
        FindWindowW(class.as_ptr(), title.as_ptr())
    };
    if result.is_null() {
        Err(String::from("cannot find window"))
    } else {
        Ok(result)
    }
}

pub fn find_window_cloud() -> Result<HWND, String> {
    let title = encode_wide(String::from("云·原神"));
    //let class = encode_wide(String::from("Qt5152QWindowIcon"));
    unsafe {
        let mut result: HWND = null_mut();
        for i in 0..3 {
            result = FindWindowExW(null_mut(), result, null_mut(), title.as_ptr());
            let exstyle = GetWindowLongPtrW(result, GWL_EXSTYLE);
            let style = GetWindowLongPtrW(result, GWL_STYLE);
            if exstyle == 0x0 && style == 0x96080000 {
                return Ok(result);//全屏
            } else if exstyle == 0x100 && style == 0x96CE0000 {
                return  Ok(result);//窗口
            }
        }
    }
    Err(String::from("cannot find window"))
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

    let mut point: WinPoint = WinPoint {
        x: 0,
        y: 0,
    };
    ClientToScreen(hwnd, &mut point as *mut WinPoint);
    let left: i32 = point.x;
    let top: i32 = point.y;

    Ok(PixelRect {
        left, top,
        width, height
    })
}

pub fn get_client_rect(hwnd: HWND) -> Result<PixelRect, String> {
    unsafe {
        get_client_rect_unsafe(hwnd)
    }
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
    stdin().read_line(&mut s);
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
        0, 0, 0, 0, 0, 0,
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
    unsafe {
        is_admin_unsafe()
    }
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

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_version() -> String {
    VERSION.into()
}

pub fn check_update() -> Option<String> {
    let client = Client::new();

    let resp = client.get("https://api.github.com/repos/wormtql/yas/tags")
        .timeout(Duration::from_secs(5))
        .header(USER_AGENT, HeaderValue::from_static("reqwest"))
        .send().ok()?.json::<Vec<GithubTag>>().ok()?;

    let latest = if resp.len() == 0 {
        return None
    } else {
        resp[0].name.clone()
    };
    let latest = &latest[1..];

    let latest_sem: semver::Version = semver::Version::parse(&latest).unwrap();
    let current_sem: semver::Version = semver::Version::parse(&get_version()).unwrap();

    if latest_sem > current_sem {
        Some(String::from(latest))
    } else {
        None
    }
}

pub fn encode_lpcstr(s: &str) -> Vec<i8> {
    let mut arr: Vec<i8> = s.bytes().map(|x| x as i8).collect();
    arr.push(0);
    arr
}

pub fn set_dpi_awareness() {
    // let os = os_info::get();

    let h_lib = unsafe {
        // let names = ["SHCore.dll"]

        LoadLibraryA(encode_lpcstr("Shcore.dll").as_ptr())
    };
    if h_lib.is_null() {
        info!("`Shcore.dll` not found");
        unsafe {
            SetProcessDPIAware();
        }
    } else {
        info!("`Shcore.dll` found");
        unsafe {
            let addr = GetProcAddress(h_lib, encode_lpcstr("SetProcessDpiAwareness").as_ptr());
            if addr.is_null() {
                warn!("cannot find process `SetProcessDpiAwareness`, but `Shcore.dll` exists");
                SetProcessDPIAware();
            } else {
                // func(PROCESS_DPI_AWARENESS) -> HRESULT
                let func = transmute::<*const (), fn(u32) -> i32>(addr as *const ());
                // SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
                func(2);
            }

            FreeLibrary(h_lib);
        }
    }



    // if os.version() >= &os_info::Version::from_string("8.1") {
    //     info!("Windows version >= 8.1");
    //     unsafe  {
    //         SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    //     }
    // } else {
    //     info!("Windows version < 8.1");
    //     unsafe {
    //         SetProcessDPIAware();
    //     }
    // }
}

pub fn show_window_and_set_foreground(hwnd: HWND) {
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd);
    }
}
