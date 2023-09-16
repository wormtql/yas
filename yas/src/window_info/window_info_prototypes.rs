use std::collections::HashMap;

use crate::game_info::game_info::Resolution;

use super::window_info::WindowInfo;

pub struct WindowInfoPrototypes {
    pub data: HashMap<Resolution, WindowInfo>
}

impl WindowInfoPrototypes {
    pub fn get_window_info(&self, res: Resolution) -> Option<&WindowInfo> {
        self.data.get(&res)
    }
}