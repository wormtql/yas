use std::collections::HashMap;
use crate::{common::positioning::{Rect, Pos, Size, Scalable}, game_info::Resolution};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use crate::window_info::window_info_type::WindowInfoType;

/// Maps a window-info-key to a list of entries
/// where entries consist of a size where the value is recorded, and accordingly a value
#[derive(Serialize, Deserialize, Clone)]
pub struct WindowInfoRepository {
    pub data: HashMap<String, HashMap<Size, WindowInfoType>>,
}

impl WindowInfoRepository {
    pub fn new() -> WindowInfoRepository {
        WindowInfoRepository {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, size: Size, value: WindowInfoType) {
        self.data
            .entry(String::from(name))
            .or_insert(HashMap::new())
            .insert(size, value);
    }

    pub fn add_pos(&mut self, name: &str, size: Size, value: Pos) {
        self.data
            .entry(String::from(name))
            .or_insert(HashMap::new())
            .insert(size, WindowInfoType::Pos(value));
    }

    pub fn merge_inplace(&mut self, other: &WindowInfoRepository) {
        for (key, data) in other.data.iter() {
            if self.data.contains_key(key) {
                for (resolution, value) in data.iter() {
                    self.data[key].insert(resolution.clone(), value.clone());
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
    pub fn get_exact<T>(&self, name: &str, window_size: Size) -> Option<T> where WindowInfoType: TryInto<T> {
        if self.data.contains_key(name) {
            if self.data[name].contains_key(&window_size) {
                return self.data[name][&window_size].try_into().ok();
            }
        }

        None
    }

    /// Get window info by name and size
    /// if window size does not exists exactly, this function will search for the same resolution family and scale the result
    pub fn get_auto_scale<T>(&self, name: &str, window_size: Size) -> Option<T> where WindowInfoType: TryInto<T> {
        if self.data.contains_key(name) {
            if self.data[name].contains_key(&window_size) {
                return self.data[name][&window_size].try_into().ok();
            } else {
                // todo find a biggest size which can be scaled, this will reduce error
                // find if a resolution can be scaled
                for (size, value) in self.data[name].iter() {
                    if size.width * window_size.height == size.height * window_size.width {
                        // can be scaled
                        let factor: f64 = size.width / window_size.width;
                        return Some(value.scale(factor));
                    }
                }
            }
        }

        None
    }
}
