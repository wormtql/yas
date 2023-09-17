use std::collections::HashMap;

use crate::game_info::game_info::Resolution;

use super::window_info::WindowInfo;

pub struct WindowInfoPrototypes {
    pub data: HashMap<Resolution, WindowInfo>
}

// constructors
impl WindowInfoPrototypes {
    pub fn new() -> Self {
        WindowInfoPrototypes {
            data: HashMap::new()
        }
    }
}

impl WindowInfoPrototypes {
    pub fn get_window_info(&self, res: Resolution) -> Option<&WindowInfo> {
        self.data.get(&res)
    }

    pub fn insert(&mut self, window_info: WindowInfo) {
        self.data.insert(window_info.resolution_family, window_info);
    }
}