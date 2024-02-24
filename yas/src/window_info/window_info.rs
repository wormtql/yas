use std::collections::HashMap;
use crate::{common::positioning::{Rect, Pos, Size, Scalable}, game_info::Resolution};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

type StdResult<T, E> = std::result::Result<T, E>;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum WindowInfoType {
    Rect(Rect),
    Pos(Pos),
    Size(Size),
    Float(f64),
    /// invariant means, when window size scales, this amount will not scale
    InvariantInt(i32),
    InvariantFloat(f64),
}

// due to orphan rule, we implement TryInto instead of TryFrom
impl TryInto<i32> for WindowInfoType {
    type Error = String;

    fn try_into(self) -> std::result::Result<i32, Self::Error> {
        match self {
            WindowInfoType::InvariantInt(v) => StdResult::Ok(v),
            _ => StdResult::Err(String::from("not an i32 type"))
        }
    }
}

impl TryInto<Rect> for WindowInfoType {
    type Error = String;

    fn try_into(self) -> std::result::Result<Rect, Self::Error> {
        match self {
            WindowInfoType::Rect(rect) => StdResult::Ok(rect),
            _ => StdResult::Err(String::from("not a rect type")),
        }
    }
}

impl TryInto<Pos> for WindowInfoType {
    type Error = String;

    fn try_into(self) -> std::result::Result<Pos, Self::Error> {
        match self {
            WindowInfoType::Pos(pos) => StdResult::Ok(pos),
            _ => StdResult::Err(String::from("not a pos type")),
        }
    }
}

impl TryInto<f64> for WindowInfoType {
    type Error = String;

    fn try_into(self) -> std::result::Result<f64, Self::Error> {
        match self {
            WindowInfoType::Float(f) => StdResult::Ok(f),
            WindowInfoType::InvariantFloat(f) => StdResult::Ok(f),
            _ => StdResult::Err(String::from("not a float type")),
        }
    }
}

impl TryInto<Size> for WindowInfoType {
    type Error = String;

    fn try_into(self) -> std::result::Result<Size, Self::Error> {
        match self {
            WindowInfoType::Size(size) => StdResult::Ok(size),
            _ => StdResult::Err(String::from("not a size type")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WindowInfo {
    pub data: HashMap<String, WindowInfoType>,
    pub current_resolution: Size,
    pub resolution_family: Resolution,
}

impl Scalable for WindowInfoType {
    fn scale(&self, factor: f64) -> Self {
        let result = match *self {
            WindowInfoType::Rect(rect) => WindowInfoType::Rect(rect.scale(factor)),
            WindowInfoType::Pos(pos) => WindowInfoType::Pos(pos.scale(factor)),
            WindowInfoType::Size(size) => WindowInfoType::Size(size.scale(factor)),
            WindowInfoType::Float(v) => WindowInfoType::Float(v.scale(factor)),
            WindowInfoType::InvariantInt(v) => WindowInfoType::InvariantInt(v),
            WindowInfoType::InvariantFloat(v) => WindowInfoType::InvariantFloat(v),
        };
        result
    }
}

impl Scalable for WindowInfo {
    fn scale(&self, factor: f64) -> Self {
        let mut result = WindowInfo {
            data: HashMap::new(),
            current_resolution: self.current_resolution.scale(factor),
            resolution_family: self.resolution_family
        };
        for (k, &v) in self.data.iter() {
            result.data.insert(k.clone(), v);
        }
        result
    }
}

impl WindowInfo {
    pub fn new(current_resolution: Size, resolution_family: Resolution) -> WindowInfo {
        WindowInfo {
            data: HashMap::new(),
            current_resolution,
            resolution_family
        }
    }

    pub fn add_pos(&mut self, name: &str, value: Pos) {
        self.data.insert(String::from(name), WindowInfoType::Pos(value));
    }

    pub fn merge(&self, other: &WindowInfo) -> Result<WindowInfo> {
        if self.resolution_family != other.resolution_family {
            return Err(anyhow!("resolution family not match"));
        }
        let mut result = WindowInfo {
            data: self.data.clone(),
            current_resolution: self.current_resolution,
            resolution_family: self.resolution_family
        };

        for (name, value) in other.data.iter() {
            result.data.insert(name.clone(), value.clone());
        }
        
        anyhow::Ok(result)
    }

    pub fn get<T>(&self, name: &str) -> Option<T> where WindowInfoType: TryInto<T> {
        let result = self.data.get(name).cloned();
        if result.is_none() {
            return None;
        }

        let window_info_type = result.unwrap();
        let result: StdResult<T, _> = window_info_type.try_into();
        result.ok()
    }
}

#[macro_export]
macro_rules! load_window_info {
    ($filename:expr) => {
        {
            let s = include_str!($filename);
            let result: WindowInfo = serde_json::from_str(s).unwrap();
            result
        }
    };
}