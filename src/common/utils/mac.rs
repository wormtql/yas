use enigo::*;
use crate::common::{utils, PixelRect};

#[cfg(target_arch = "aarch64")]
pub fn mac_scroll(enigo:&mut Enigo, count:i32) {
    enigo.mouse_down(MouseButton::Left);
    utils::sleep(10);
    for j in 0..count {
        for i in 0..5 {
            enigo.mouse_move_relative(0, -2);
            utils::sleep(10);
        }
        enigo.mouse_up(MouseButton::Left);
        utils::sleep(10);
        enigo.mouse_move_relative(0, 10);
        utils::sleep(10);
    }
}

#[cfg(target_arch = "x86")]
pub fn mac_scroll(count:i32) {
    self.enigo.mouse_scroll_y(count);
    utils::sleep(20);
}

pub fn get_pid() -> i32 {
    let pid_str = unsafe{
    String::from_utf8_unchecked(
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&format!(r#"ps -Aj | grep [Y]uanshen | cut -f 2 -w"#))
            .output()
            .unwrap()
            .stdout,
    )};
    pid_str.trim().parse::<i32>().unwrap() as i32
}


pub unsafe fn find_window_by_pid(pid:i32) -> Result<(PixelRect, String), String>{
    use core_foundation::string::{CFString, CFStringRef};
    use core_foundation::number::{CFNumber, CFNumberRef};
    use core_foundation::base::TCFType;
    use core_foundation::dictionary::{CFDictionaryGetValueIfPresent, CFDictionary, CFDictionaryRef};
    use core_foundation::array::{CFArrayGetValueAtIndex, CFArrayGetCount};
    use std::ffi::c_void;
    use core_graphics::geometry::CGRect;
    use core_graphics::window::{CGWindowListCopyWindowInfo, kCGWindowListOptionExcludeDesktopElements, kCGNullWindowID, kCGWindowOwnerPID, kCGWindowBounds, kCGWindowOwnerName};


    let cf_win_array = CGWindowListCopyWindowInfo(kCGWindowListOptionExcludeDesktopElements, kCGNullWindowID);
    let count = CFArrayGetCount(cf_win_array);
    if count == 0 {
        return Err("No genshin window found".to_string());
    }

    let mut mrect = PixelRect{left:0, top:0, width:0, height:0};
    let mut window_count = 0;
    let mut title:String = String::new();

    for i in 0..count {
        let win_info_ref:CFDictionaryRef = CFArrayGetValueAtIndex(cf_win_array, i) as CFDictionaryRef;
        let mut test_pid_ref: *const c_void = std::ptr::null_mut();
        assert!(CFDictionaryGetValueIfPresent(win_info_ref, kCGWindowOwnerPID as *const c_void, &mut test_pid_ref)!=0);
        let test_pid = CFNumber::wrap_under_get_rule(test_pid_ref as CFNumberRef);


        if pid == test_pid.to_i32().unwrap() {
            let mut cg_bounds_dict_ref: *const c_void = std::ptr::null_mut();
            CFDictionaryGetValueIfPresent(win_info_ref, kCGWindowBounds as *const c_void, &mut cg_bounds_dict_ref);
            let cg_bounds_dict = CFDictionary::wrap_under_get_rule(cg_bounds_dict_ref as CFDictionaryRef);
            let cg_rect = CGRect::from_dict_representation(&cg_bounds_dict).unwrap();

            let mut cg_title_ref: *const c_void = std::ptr::null_mut();
            CFDictionaryGetValueIfPresent(win_info_ref, kCGWindowOwnerName as *const c_void, &mut cg_title_ref);
            let cg_title = CFString::wrap_under_get_rule(cg_title_ref as CFStringRef);
            title = cg_title.to_string();
            if cg_rect.origin.x > 0.0 {
                mrect = PixelRect {
                    left:cg_rect.origin.x as i32,
                    top:cg_rect.origin.y as i32,
                    width:cg_rect.size.width as i32,
                    height:cg_rect.size.height as i32,
                };
                window_count+=1
            }
        }
    }
    if window_count > 0 {
        Ok((mrect, title))
    } else {
        Err("No genshin window found".to_string())
    }

}