use std::collections::HashMap;

use crate::game_info::Resolution;

use super::window_info_repository::WindowInfoRepository;

pub struct WindowInfoPrototypes {
    pub data: HashMap<Resolution, WindowInfoRepository>
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
    pub fn get_window_info(&self, res: Resolution) -> Option<&WindowInfoRepository> {
        self.data.get(&res)
    }

    pub fn insert(&mut self, window_info: WindowInfoRepository) {
        self.data.insert(window_info.resolution_family, window_info);
    }
}