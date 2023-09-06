use crate::{common::UI, core::ui::os::GameInfo, *};
use winapi::shared::windef::HWND;

fn window_not_found() -> ! {
    crate::error_and_quit!(
        "未找到游戏窗口，请确认{}已经开启",
        TARGET_GAME.get().unwrap()
    )
}

fn get_game_window_name() -> &'static [&'static str] {
    match TARGET_GAME.get().unwrap() {
        Game::Genshin => &["原神", "Genshin Impact"],
        Game::StarRail => &["崩坏：星穹铁道", "Honkai: Star Rail"],
        _ => unimplemented!("不支持的游戏"),
    }
}

fn get_cloud_window_name() -> &'static [&'static str] {
    match TARGET_GAME.get().unwrap() {
        Game::Genshin => &["云·原神"],
        _ => unimplemented!("不支持的游戏"),
    }
}

fn get_window() -> (HWND, bool) {
    for name in get_game_window_name() {
        let hwnd = utils::find_window_local(name);
        if hwnd.is_ok() {
            return (hwnd, false);
        }
    }

    for name in get_cloud_window_name() {
        let hwnd = utils::find_window_local(name);
        if hwnd.is_ok() {
            return (hwnd, true);
        }
    }

    window_not_found()
}

pub fn get_game_info() -> GameInfo {
    use winapi::um::winuser::{SetForegroundWindow, ShowWindow, SW_RESTORE};

    utils::set_dpi_awareness();

    let (hwnd, is_cloud) = get_window().unwrap();

    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }

    unsafe {
        SetForegroundWindow(hwnd);
    }

    utils::sleep(1000);

    let rect = utils::get_client_rect(hwnd).unwrap();

    GameInfo {
        window_pos: rect,
        size: WindowSize::new(rect.size),
        is_cloud,
        ui: UI::Desktop,
    }
}
