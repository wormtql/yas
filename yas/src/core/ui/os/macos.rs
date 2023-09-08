use crate::{common::utils::*, core::ui::Resolution};

use super::GameInfo;

pub fn get_game_info() -> GameInfo {
    let (pid, ui) = get_pid_and_ui();

    let (rect, window_title) = unsafe { find_window_by_pid(pid).unwrap() };

    info!("找到游戏窗口：{} (PID: {})", window_title, pid);

    GameInfo {
        window: rect,
        resolution: Resolution::new(rect.size),
        is_cloud: false,
        ui,
    }
}
