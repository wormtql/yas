use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_info::{Platform, UI};
use crate::positioning::Size;
use crate::window_info::WindowInfoType;
use crate::window_info::WindowInfoRepository;

/// Which is a format, where the whole file are recorded under a certain resolution
#[derive(Serialize, Deserialize)]
pub struct WindowInfoTemplatePerSize {
    pub current_resolution: Size<usize>,
    pub platform: Platform,
    pub ui: UI,
    pub data: HashMap<String, WindowInfoType>
}

impl WindowInfoTemplatePerSize {
    pub fn inject_into_window_info_repo(&self, repo: &mut WindowInfoRepository) {
        for (name, value) in self.data.iter() {
            repo.add(&name, self.current_resolution, self.ui, self.platform, *value);
        }
    }
}

pub macro load_window_info_repo($($filename:literal),+ $(,)?) {
    {
        let mut result = WindowInfoRepository::new();
        $(
            {
                let s = include_str!($filename);
                let f: WindowInfoTemplatePerSize = serde_json::from_str(&s).unwrap();
                f.inject_into_window_info_repo(&mut result);
            }
        )*
        result
    }
}

