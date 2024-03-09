use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::game_info::{Platform, UI};
use crate::positioning::{Pos, Scalable, Size};

use crate::window_info::WindowInfoType;

/// Maps a window-info-key to a list of entries
/// where entries consist of a size where the value is recorded, and accordingly a value
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowInfoRepository {
    /// window info key -> (window size, ui, platform)
    pub data: HashMap<String, HashMap<(Size<usize>, UI, Platform), WindowInfoType>>,
}

impl WindowInfoRepository {
    pub fn new() -> WindowInfoRepository {
        WindowInfoRepository {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, size: Size<usize>, ui: UI, platform: Platform, value: WindowInfoType) {
        self.data
            .entry(String::from(name))
            .or_insert(HashMap::new())
            .insert((size, ui, platform), value);
    }

    pub fn add_pos(&mut self, name: &str, size: Size<usize>, ui: UI, platform: Platform, value: Pos<f64>) {
        self.data
            .entry(String::from(name))
            .or_insert(HashMap::new())
            .insert((size, ui, platform), WindowInfoType::Pos(value));
    }

    pub fn merge_inplace(&mut self, other: &WindowInfoRepository) {
        for (key, data) in other.data.iter() {
            if self.data.contains_key(key) {
                for (resolution, value) in data.iter() {
                    self.data.get_mut(key).unwrap().insert(resolution.clone(), value.clone());
                }
            } else {
                self.data.insert(key.clone(), data.clone());
            }
        }
    }

    pub fn merge(&self, other: &WindowInfoRepository) -> WindowInfoRepository {
        let mut result = self.clone();
        result.merge_inplace(other);
        result
    }

    /// Get window info by name and size
    /// if name or resolution does not exist, then return None
    pub fn get_exact<T>(&self, name: &str, window_size: Size<usize>, ui: UI, platform: Platform) -> Option<T> where WindowInfoType: TryInto<T> {
        if self.data.contains_key(name) {
            if self.data[name].contains_key(&(window_size, ui, platform)) {
                return self.data[name][&(window_size, ui, platform)].try_into().ok();
            }
        }

        None
    }

    /// Get window info by name and size
    /// if window size does not exists exactly, this function will search for the same resolution family and scale the result
    pub fn get_auto_scale<T>(&self, name: &str, window_size: Size<usize>, ui: UI, platform: Platform) -> Option<T> where WindowInfoType: TryInto<T> {
        if self.data.contains_key(name) {
            if self.data[name].contains_key(&(window_size, ui, platform)) {
                return self.data[name][&(window_size, ui, platform)].try_into().ok();
            } else {
                // todo find a biggest size which can be scaled, this will reduce error
                // find if a resolution can be scaled
                for (k, value) in self.data[name].iter() {
                    let size = &k.0;
                    if size.width * window_size.height == size.height * window_size.width
                        && k.1 == ui && k.2 == platform
                    {
                        let factor: f64 = window_size.width as f64 / size.width as f64;
                        return value.scale(factor).try_into().ok();
                    }
                }
            }
        }

        None
    }
}
