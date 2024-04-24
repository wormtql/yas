use serde::{Deserialize, Serialize};
use crate::positioning::{Pos, Rect, Scalable, Size};
use anyhow::anyhow;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum WindowInfoType {
    Rect(Rect<f64>),
    Pos(Pos<f64>),
    Size(Size<f64>),
    Float(f64),
    /// when window size scales, these amount will not scale
    InvariantInt(i32),
    InvariantFloat(f64),
}

// due to orphan rule, we implement TryInto instead of TryFrom
impl TryInto<i32> for WindowInfoType {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            WindowInfoType::InvariantInt(v) => Ok(v),
            _ => Err(anyhow!(String::from("not an i32 type")))
        }
    }
}

impl TryInto<Rect<f64>> for WindowInfoType {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Rect<f64>, Self::Error> {
        match self {
            WindowInfoType::Rect(rect) => Ok(rect),
            _ => Err(anyhow!(String::from("not a rect type"))),
        }
    }
}

impl TryInto<Pos<f64>> for WindowInfoType {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Pos<f64>, Self::Error> {
        match self {
            WindowInfoType::Pos(pos) => Ok(pos),
            _ => Err(anyhow!(String::from("not a pos type"))),
        }
    }
}

impl TryInto<f64> for WindowInfoType {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<f64, Self::Error> {
        match self {
            WindowInfoType::Float(f) => Ok(f),
            WindowInfoType::InvariantFloat(f) => Ok(f),
            _ => Err(anyhow!(String::from("not a float type"))),
        }
    }
}

impl TryInto<Size<f64>> for WindowInfoType {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Size<f64>, Self::Error> {
        match self {
            WindowInfoType::Size(size) => Ok(size),
            _ => Err(anyhow!(String::from("not a size type"))),
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