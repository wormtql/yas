use std::collections::HashMap;
use std::ops::Mul;
use crate::{common::positioning::{Rect, Pos, Size, Scalable}, game_info::game_info::Resolution};
use anyhow::{Result, anyhow};

type StdResult = std::result::Result;

#[derive(Copy, Clone, Debug)]
pub enum WindowInfoType {
    Rect(Rect),
    Pos(Pos),
    Float(f64),
    InvariantInt(i32),
    InvariantFloat(f64),
}

// due to orphan rule, we implement TryInto instead of TryFrom 
impl TryInto<Rect> for WindowInfoType {
    fn try_into(self) -> std::result::Result<Rect, Self::Error> {
        match *self {
            WindowInfoType::Rect(rect) => StdResult::Ok(rect),
            _ => StdResult::Err(String::from("not a rect type")),
        }
    }
}

impl TryInto<Pos> for WindowInfoType {
    fn try_into(self) -> std::result::Result<Pos, Self::Error> {
        match *self {
            WindowInfoType::Pos(pos) => StdResult::Ok(pos),
            _ => StdResult::Err(String::from("not a pos type")),
        }
    }
}

impl TryInto<f64> for WindowInfoType {
    fn try_into(self) -> std::result::Result<f64, Self::Error> {
        match *self {
            WindowInfoType::Float(f) => StdResult::Ok(f),
            WindowInfoType::InvariantFloat(f) => StdResult::Ok(f),
            _ => StdResult::Err(String::from("not a float type")),
        }
    }
}

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
            WindowInfoType::Number(v) => WindowInfoType::Number(v.scale(factor)),
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
    pub fn merge(&self, other: &WindowInfo) -> Result<WindowInfo> {
        if (self.resolution_family != other.resolution_family) {
            return anyhow!("resolution family not match");
        }
        let mut result = WindowInfo {
            data: self.data.clone(),
            current_resolution: self.current_resolution,
            resolution_family: self.resolution_family
        };
        result.data.extend(other.data.iter());
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

