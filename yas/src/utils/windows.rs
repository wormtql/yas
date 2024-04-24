use std::ffi::OsStr;
use std::iter::once;
use std::mem::transmute;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use anyhow::{anyhow, Result};
use log::{info, warn};
use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::{HWND, POINT as WinPoint, RECT as WinRect};
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA};
use winapi::um::securitybaseapi::{AllocateAndInitializeSid, CheckTokenMembership, FreeSid};
use winapi::um::winnt::{
    DOMAIN_ALIAS_RID_ADMINS, PSID, SECURITY_BUILTIN_DOMAIN_RID, SECURITY_NT_AUTHORITY,
    SID_IDENTIFIER_AUTHORITY,
};
use winapi::um::winuser::{
    ClientToScreen, FindWindowExW, FindWindowW, GetAsyncKeyState, GetClientRect, GetWindowLongPtrW,
    GWL_EXSTYLE, GWL_STYLE, SetForegroundWindow, SetProcessDPIAware, ShowWindow, SW_RESTORE,
    VK_RBUTTON,
};

use crate::positioning::Rect;

pub fn encode_lpcstr(s: &str) -> Vec<i8> {
    let mut arr: Vec<i8> = s.bytes().map(|x| x as i8).collect();
    arr.push(0);
    arr
}

fn encode_wide_with_null(s: impl AsRef<str>) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(s.as_ref())
        .encode_wide()
        .chain(once(0))
        .collect();
    wide
}

pub fn find_window_local(title: impl AsRef<str>) -> Result<HWND> {
    let title = encode_wide_with_null(title);
    let class = encode_wide_with_null("UnityWndClass");
    let result: HWND = unsafe { FindWindowW(class.as_ptr(), title.as_ptr()) };
    if result.is_null() {
        Err(anyhow!("cannot find window"))
    } else {
        Ok(result)
    }
}

pub fn find_window_cloud() -> Result<HWND> {
    let title = encode_wide_with_null(String::from("云·原神"));
    //let class = encode_wide(String::from("Qt5152QWindowIcon"));
    unsafe {
        let mut result: HWND = null_mut();
        for _ in 0..3 {
            result = FindWindowExW(null_mut(), result, null_mut(), title.as_ptr());
            let exstyle = GetWindowLongPtrW(result, GWL_EXSTYLE);
            let style = GetWindowLongPtrW(result, GWL_STYLE);
            if exstyle == 0x0 && style == 0x96080000 {
                return Ok(result); //全屏
            } else if exstyle == 0x100 && style == 0x96CE0000 {
                return Ok(result); //窗口
            }
        }
    }
    Err(anyhow!("cannot find window"))
}

unsafe fn get_client_rect_unsafe(hwnd: HWND) -> Result<Rect<i32>> {
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

    Ok(Rect {
        left,
        top,
        width,
        height
    })
}

pub fn get_client_rect(hwnd: HWND) -> Result<Rect<i32>> {
    unsafe { get_client_rect_unsafe(hwnd) }
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

pub fn set_dpi_awareness() {
    let h_lib = unsafe {
        LoadLibraryA(encode_lpcstr("Shcore.dll").as_ptr())
    };
    if h_lib.is_null() {
        unsafe {
            SetProcessDPIAware();
        }
    } else {
        unsafe {
            let addr = GetProcAddress(h_lib, encode_lpcstr("SetProcessDpiAwareness").as_ptr());
            if addr.is_null() {
                warn!("cannot find process `SetProcessDpiAwareness`, but `Shcore.dll` exists");
                SetProcessDPIAware();
            } else {
                let func = transmute::<*const (), fn(u32) -> i32>(addr as *const ());
                func(2);
            }

            FreeLibrary(h_lib);
        }
    }
}

pub fn show_window_and_set_foreground(hwnd: HWND) {
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd);
    }
}
