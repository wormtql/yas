use crate::game_info::game_info::{GameInfo, Resolution, UI};
use crate::utils;
use winapi::shared::windef::HWND;

fn window_not_found(game_name: &str) -> ! {
    crate::error_and_quit!(
        "未找到游戏窗口，请确认{}已经开启",
        game_name
    )
}

// fn get_game_window_name() -> &'static [&'static str] {
//     match crate::TARGET_GAME.get().unwrap() {
//         Game::Genshin => &["原神", "Genshin Impact"],
//         Game::StarRail => &["崩坏：星穹铁道", "Honkai: Star Rail"],
//         _ => unimplemented!("不支持的游戏"),
//     }
// }

// fn get_cloud_window_name() -> &'static [&'static str] {
//     match crate::TARGET_GAME.get().unwrap() {
//         Game::Genshin => &["云·原神"],
//         _ => unimplemented!("不支持的游戏"),
//     }
// }

fn get_window(window_names: &[&str]) -> (HWND, bool) {
    // local game names
    // let local_game_names = ["原神", "Genshin Impact"];
    for name in window_names.iter() {
        let hwnd = utils::find_window_local(name);
        if let Ok(hwnd) = hwnd {
            return (hwnd, false);
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

    window_not_found("op")
}

pub fn get_game_info(window_names: &[&str]) -> GameInfo {
    use winapi::um::winuser::{SetForegroundWindow, ShowWindow, SW_RESTORE};

    utils::set_dpi_awareness();

    let (hwnd, is_cloud) = get_window(window_names);

    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }

    unsafe {
        SetForegroundWindow(hwnd);
    }

    utils::sleep(1000);

    let rect = utils::get_client_rect(hwnd).unwrap();

    GameInfo {
        window: rect,
        resolution: Resolution::new(rect.size),
        is_cloud,
        ui: UI::Desktop,
    }
}
