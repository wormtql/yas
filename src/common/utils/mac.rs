use crate::common::{utils, PixelRect, UI};
use cocoa::{
    appkit::CGFloat,
    base::NO,
    foundation::{NSAutoreleasePool, NSPoint, NSSize},
};
use enigo::*;

#[cfg(target_arch = "aarch64")]
pub fn mac_scroll(enigo: &mut Enigo, count: i32) {
    enigo.mouse_down(MouseButton::Left);
    utils::sleep(10);
    for j in 0..count {
        for i in 0..5 {
            enigo.mouse_move_relative(0, -2);
            utils::sleep(20);
        }
        enigo.mouse_up(MouseButton::Left);
        utils::sleep(10);
        enigo.mouse_move_relative(0, 10);
        utils::sleep(20);
    }
}

#[cfg(target_arch = "x86")]
pub fn mac_scroll(count: i32) {
    self.enigo.mouse_scroll_y(count);
    utils::sleep(20);
}

pub fn get_titlebar_height() -> f64 {
    use cocoa::appkit::{NSBackingStoreBuffered, NSImage, NSWindow, NSWindowStyleMask};
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
                .arg(&format!(r#"ps -Aj | grep [Y]uanshen | cut -f 2 -w"#))
                .output()
                .unwrap()
                .stdout,
        )
    };

    let pid_str_genshin_wine = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    r#"top -l 1 -o mem | grep wine64-preloader | head -n 1 | sed 's/^[ ]*//' | cut -d ' ' -f 1"#
                ))
                .output()
                .unwrap()
                .stdout,
        )
    };

    match pid_str_playcover.trim().parse::<i32>() {
        Ok(pid) => (pid, UI::Mobile),
        Err(_) => match pid_str_genshin_wine.trim().parse::<i32>() {
            Ok(pid) => (pid, UI::Desktop),
            Err(_) => panic!("No genshin program found"),
        },
    }
}

pub unsafe fn find_window_by_pid(pid: i32) -> Result<(PixelRect, String), String> {
    use cocoa::appkit::{NSBackingStoreBuffered, NSImage, NSWindow, NSWindowStyleMask};
    use cocoa::base::nil;
    use cocoa::foundation::NSRect;
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
        return Err("No genshin window found".to_string());
    }

    let mut mrect = PixelRect {
        left: 0,
        top: 0,
        width: 0,
        height: 0,
    };
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

            // mac app cg_rect.size.x = 0 when full screen
            if cg_rect.size.height > 200. {
                if cg_rect.origin.y > 0. {
                    // Window Mode
                    let titlebar_height = get_titlebar_height();
                    mrect = PixelRect {
                        left: cg_rect.origin.x as i32,
                        top: cg_rect.origin.y as i32 + titlebar_height as i32, // The titlebar appears in windowe mode
                        width: cg_rect.size.width as i32,
                        height: cg_rect.size.height as i32 - titlebar_height as i32, // The titlebar appears in windowe mode
                    };
                } else {
                    mrect = PixelRect {
                        left: cg_rect.origin.x as i32,
                        top: cg_rect.origin.y as i32,
                        width: cg_rect.size.width as i32,
                        height: cg_rect.size.height as i32,
                    };
                }
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
