use crate::game_info::{GameInfo, ResolutionFamily, UI, Platform};
use crate::utils;
use anyhow::{Result, anyhow};
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

fn get_window(window_names: &[&str]) -> Result<(HWND, bool)> {
    let handles = utils::iterate_window();
    for hwnd in handles.iter() {
        let title = utils::get_window_title(*hwnd);
        if let Some(t) = title {
            let trimmed = t.trim();

            for name in window_names.iter() {
                if trimmed == *name {
                    return Ok((*hwnd, false));
                }
            }
        }
    }

    // cloud games
    // let cloud_game_names = [""]
    // for name in get_cloud_window_name() {
    //     let hwnd = utils::find_window_local(name);
    //     if let Ok(hwnd) = hwnd {
    //         return (hwnd, true);
    //     }
    // }

    Err(anyhow!("未找到游戏窗口，请确认{:?}已经开启", window_names))
}

pub fn get_game_info(window_names: &[&str]) -> Result<GameInfo> {
    utils::set_dpi_awareness();

    let (hwnd, is_cloud) = get_window(window_names)?;

    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }

    unsafe {
        SetForegroundWindow(hwnd);
    }

    utils::sleep(1000);

    let rect = utils::get_client_rect(hwnd)?;

    Ok(GameInfo {
        window: rect,
        resolution_family: ResolutionFamily::new(rect.to_rect_usize().size()).unwrap(),
        is_cloud,
        ui: UI::Desktop,
        platform: Platform::Windows
    })
}
