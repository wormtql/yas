use serde::{Deserialize, Serialize};
use crate::common::positioning::{Pos, Rect, Scalable, Size};

type StdResult<T, E> = std::result::Result<T, E>;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum WindowInfoType {
    Rect(Rect),
    Pos(Pos),
    Size(Size),
    Float(f64),
    /// when window size scales, these amount will not scale
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