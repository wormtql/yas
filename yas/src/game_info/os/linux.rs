use anyhow::{Result, anyhow};

use crate::game_info::{GameInfo, Platform, UI, ResolutionFamily};
use crate::positioning::Rect;

pub fn get_game_info() -> Result<GameInfo> {
    let window_id = String::from_utf8(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(r#" xwininfo|grep "Window id"|cut -d " " -f 4 "#)
                .output()
                .unwrap()
                .stdout,
        )?;
    let window_id = window_id.trim_end_matches("\n");

    let position_size = String::from_utf8(
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&format!(r#" xwininfo -id {window_id}|cut -f 2 -d :|tr -cd "0-9\n"|grep -v "^$"|sed -n "1,2p;5,6p" "#))
                .output()
                .unwrap()
                .stdout,
        )?;

    let mut info = position_size.split("\n");

    let left = info.next().unwrap().parse().unwrap();
    let top = info.next().unwrap().parse().unwrap();
    let width = info.next().unwrap().parse().unwrap();
    let height = info.next().unwrap().parse().unwrap();

    let rect = Rect::new(left, top, width, height);
    let rf = ResolutionFamily::new(rect.size()).ok_or(anyhow!("unknown resolution family"))?;

    Ok(GameInfo {
        window: rect.to_rect_i32(),
        resolution_family: rf,
        is_cloud: false,
        ui: UI::Desktop,
        platform: Platform::Linux,
    })
}
