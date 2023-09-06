use crate::common::utils::*;

use super::GameInfo;

pub fn get_game_window() -> GameInfo {
    let (pid, ui) = get_pid_and_ui();

    let (rect, window_title) = unsafe { find_window_by_pid(pid).unwrap() };

    info!("Found genshin pid: {}, window name: {}", pid, window_title);

    GameInfo {
        window_pos: rect,
        is_cloud: false,
        ui,
    }
}
