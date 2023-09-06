use super::*;
use crate::common::*;

pub fn get_game_info() -> GameInfo {
    let window_id = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(r#" xwininfo|grep "Window id"|cut -d " " -f 4 "#)
                .output()
                .unwrap()
                .stdout,
        )
    };
    let window_id = window_id.trim_end_matches("\n");

    let position_size = unsafe {
        String::from_utf8_unchecked(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&format!(r#" xwininfo -id {window_id}|cut -f 2 -d :|tr -cd "0-9\n"|grep -v "^$"|sed -n "1,2p;5,6p" "#))
                .output()
                .unwrap()
                .stdout,
        )
    };

    let mut info = position_size.split("\n");

    let left = info.next().unwrap().parse().unwrap();
    let top = info.next().unwrap().parse().unwrap();
    let width = info.next().unwrap().parse().unwrap();
    let height = info.next().unwrap().parse().unwrap();

    let rect = Rect::new(left, top, width, height);

    GameInfo {
        window: rect,
        resolution: Resolution::new(rect.size),
        is_cloud: false,
        ui: UI::Desktop,
    }
}
