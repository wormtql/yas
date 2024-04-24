use crate::game_info::{GameInfo, ResolutionFamily, UI, Platform};
use crate::utils;
use winapi::shared::windef::HWND;
use anyhow::{Result, anyhow};

fn get_window(window_names: &[&str]) -> Result<(HWND, bool)> {
    // local game names
    // let local_game_names = ["原神", "Genshin Impact"];
    for name in window_names.iter() {
        let hwnd = utils::find_window_local(name);
        if let Ok(hwnd) = hwnd {
            return Ok((hwnd, false));
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
    use winapi::um::winuser::{SetForegroundWindow, ShowWindow, SW_RESTORE};

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
