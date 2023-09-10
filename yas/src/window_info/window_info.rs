use std::collections::HashMap;
use std::ops::Mul;
use crate::{common::positioning::{Rect, Pos, Size, Scalable}, game_info::game_info::Resolution};
use anyhow::Result;

#[derive(Copy, Clone, Debug)]
pub enum WindowInfoType<T> {
    Rect(Rect<T>),
    Pos(Pos<T>),
    Number(T),
    InvariantInt(i32),
    InvariantFloat(f64),
}

pub struct WindowInfo {
    pub data: HashMap<String, WindowInfoType<f64>>,
    pub current_resolution: Size<f64>,
    pub resolution_family: Resolution,
}

impl<T, ScaleType> Scalable<ScaleType> for WindowInfoType<T>
where
    ScaleType: TryInto<T>,
    T: Copy + Mul<T>
{
    fn scale(&self, factor: ScaleType) -> Self {
        
    }
}

impl<T> WindowInfoType<T>
where
    T: Mul<T> + Copy
{
    pub fn try_scale<U: TryInto<T>>(&self, x: U) -> Result<WindowInfoType<T>> {
        let factor: T = x.try_into()?;
        
        let result = match *self {
            WindowInfoType::Rect(rect) => WindowInfoType::Rect(rect.scale(factor)),
            WindowInfoType::Pos(pos) => WindowInfoType::Pos()
        };
    }
}