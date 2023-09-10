use crate::common::*;
use cocoa::{
    appkit::CGFloat,
    base::NO,
    foundation::{NSAutoreleasePool, NSPoint, NSSize},
};
use enigo::*;

pub fn mac_scroll(enigo: &mut Enigo, length: i32, delta: i32, times: i32) {
    for _j in 0..length {
        enigo.mouse_down(MouseButton::Left);
        for _i in 0..times {
            enigo.mouse_move_relative(0, -delta);
            utils::sleep(10);
        }

        enigo.mouse_up(MouseButton::Left);
        utils::sleep(10);

        enigo.mouse_down(MouseButton::Left);
        utils::sleep(5);
        enigo.mouse_up(MouseButton::Left);
        utils::sleep(5);

        enigo.mouse_move_relative(0, times * delta);
        utils::sleep(20);
    }
}

pub fn mac_scroll_fast(enigo: &mut Enigo, length: i32) {
    mac_scroll(enigo, length, 4, 30);
}

pub fn mac_scroll_slow(enigo: &mut Enigo, length: i32) {
    mac_scroll(enigo, length, 4, 5);
}

pub fn get_titlebar_height() -> f64 {
    use cocoa::appkit::{NSBackingStoreBuffered, NSWindow, NSWindowStyleMask};
    use cocoa::base::nil;
    use cocoa::foundation::NSRect;
    let ns_point = NSPoint::new(100 as CGFloat, 100 as CGFloat);
    let ns_size = NSSize::new(100 as CGFloat, 100 as CGFloat);
    // create NSWindow
    let ns_rect = NSRect::new(ns_point, ns_size);
    let ns_window = unsafe {
        NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                ns_rect,
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease()
    };
    println!("{}", unsafe {
        ns_size.height - ns_window.contentRectForFrameRect_(ns_rect).size.height
    });
    unsafe { ns_size.height - ns_window.contentRectForFrameRect_(ns_rect).size.height }
}

pub fn get_pid_and_ui() -> (i32, UI) {
    let pid_str_playcover = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(r#"ps -Aj | grep "PlayCover/" | cut -f 2 -w | head -n 1"#)
                .output()
                .unwrap()
                .stdout,
        )
    };

    let pid_str_wine = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(r#"top -l 1 -o mem | grep wine64-preloader | head -n 1 | sed 's/^[ ]*//' | cut -d ' ' -f 1"#)
                .output()
                .unwrap()
                .stdout,
        )
    };

    match pid_str_playcover.trim().parse::<i32>() {
        Ok(pid) => (pid, UI::Mobile),
        Err(_) => match pid_str_wine.trim().parse::<i32>() {
            Ok(pid) => (pid, UI::Desktop),
            Err(_) => crate::error_and_quit!("No game program found"),
        },
    }
}

#[allow(clippy::default_constructed_unit_structs)]
pub fn request_capture_access() -> bool {
    use core_graphics::access::ScreenCaptureAccess;

    let access = ScreenCaptureAccess::default();

    access.preflight() || access.request()
}

pub unsafe fn find_window_by_pid(pid: i32) -> Result<(Rect, String), String> {
    use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex};
    use core_foundation::base::TCFType;
    use core_foundation::dictionary::{
        CFDictionary, CFDictionaryGetValueIfPresent, CFDictionaryRef,
    };
    use core_foundation::number::{CFNumber, CFNumberRef};
    use core_foundation::string::{CFString, CFStringRef};
    use core_graphics::geometry::CGRect;
    use core_graphics::window::{
        kCGNullWindowID, kCGWindowBounds, kCGWindowListOptionExcludeDesktopElements,
        kCGWindowOwnerName, kCGWindowOwnerPID, CGWindowListCopyWindowInfo,
    };

    use std::ffi::c_void;

    let cf_win_array =
        CGWindowListCopyWindowInfo(kCGWindowListOptionExcludeDesktopElements, kCGNullWindowID);
    let count = CFArrayGetCount(cf_win_array);

    if count == 0 {
        return Err("No game window found".to_string());
    }

    let mut mrect = Rect::default();
    let mut window_count = 0;
    let mut title: String = String::new();

    for i in 0..count {
        let win_info_ref: CFDictionaryRef =
            CFArrayGetValueAtIndex(cf_win_array, i) as CFDictionaryRef;
        let mut test_pid_ref: *const c_void = std::ptr::null_mut();
        assert!(
            CFDictionaryGetValueIfPresent(
                win_info_ref,
                kCGWindowOwnerPID as *const c_void,
                &mut test_pid_ref
            ) != 0
        );
        let test_pid = CFNumber::wrap_under_get_rule(test_pid_ref as CFNumberRef);

        if pid == test_pid.to_i32().unwrap() {
            let mut cg_bounds_dict_ref: *const c_void = std::ptr::null_mut();
            CFDictionaryGetValueIfPresent(
                win_info_ref,
                kCGWindowBounds as *const c_void,
                &mut cg_bounds_dict_ref,
            );
            let cg_bounds_dict =
                CFDictionary::wrap_under_get_rule(cg_bounds_dict_ref as CFDictionaryRef);
            let cg_rect = CGRect::from_dict_representation(&cg_bounds_dict).unwrap();

            let mut cg_title_ref: *const c_void = std::ptr::null_mut();
            CFDictionaryGetValueIfPresent(
                win_info_ref,
                kCGWindowOwnerName as *const c_void,
                &mut cg_title_ref,
            );
            let cg_title = CFString::wrap_under_get_rule(cg_title_ref as CFStringRef);
            title = cg_title.to_string();
            if cg_rect.size.height > 200. {
                mrect = if cg_rect.origin.y > 0. {
                    // Window Mode
                    Rect::from(cg_rect).with_titlebar(get_titlebar_height() as u32)
                } else {
                    Rect::from(cg_rect)
                };
                window_count += 1
            }
        }
    }
    if window_count > 0 {
        Ok((mrect, title))
    } else {
        Err("No genshin window found".to_string())
    }
}
