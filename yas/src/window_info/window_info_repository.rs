use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::positioning::{Pos, Size};

use crate::window_info::WindowInfoType;

/// Maps a window-info-key to a list of entries
/// where entries consist of a size where the value is recorded, and accordingly a value
#[derive(Serialize, Deserialize, Clone)]
pub struct WindowInfoRepository {
    pub data: HashMap<String, HashMap<Size<usize>, WindowInfoType>>,
}

impl WindowInfoRepository {
    pub fn new() -> WindowInfoRepository {
        WindowInfoRepository {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, size: Size<usize>, value: WindowInfoType) {
        self.data
            .entry(String::from(name))
            .or_insert(HashMap::new())
            .insert(size, value);
    }

    pub fn add_pos(&mut self, name: &str, size: Size<usize>, value: Pos<f64>) {
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
    pub fn get_exact<T>(&self, name: &str, window_size: Size<usize>) -> Option<T> where WindowInfoType: TryInto<T> {
        if self.data.contains_key(name) {
            if self.data[name].contains_key(&window_size) {
                return self.data[name][&window_size].try_into().ok();
            }
        }

        None
    }

    /// Get window info by name and size
    /// if window size does not exists exactly, this function will search for the same resolution family and scale the result
    pub fn get_auto_scale<T>(&self, name: &str, window_size: Size<usize>) -> Option<T> where WindowInfoType: TryInto<T> {
        if self.data.contains_key(name) {
            if self.data[name].contains_key(&window_size) {
                return self.data[name][&window_size].try_into().ok();
            } else {
                // todo find a biggest size which can be scaled, this will reduce error
                // find if a resolution can be scaled
                for (size, value) in self.data[name].iter() {
                    if size.width * window_size.height == size.height * window_size.width {
                        // can be scaled
                        let factor: f64 = size.width as f64 / window_size.width as f64;
                        return Some(value.scale(factor));
                    }
                }
            }
        }

        None
    }
}
