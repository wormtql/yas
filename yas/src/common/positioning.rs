use std::ops::{Add, Mul, Sub};
use std::fmt::Display;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Rect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Size {
    pub height: f64,
    pub width: f64,
}

impl Add<Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Pos> for Pos {
    type Output = Self;

    fn sub(self, rhs: Pos<T>) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Rect where T: TryInto<f64> {
    pub fn new(left: T, top: T, width: T, height: T) -> Rect {
        let left = left.try_into().unwrap();
        let top = top.try_into().unwrap();
        let width = width.try_into().unwrap();
        let height = height.try_into().unwrap();
        Rect {
            left, top, width, height
        }
    }
}

impl<T> Pos where T: TryInto<f64> {
    pub fn new(x: T, y: T) -> Pos {
        let x = x.try_into().unwrap();
        let y = y.try_into().unwrap();
        Pos {
            x, y
        }
    }
}


impl Rect where {
    pub fn translate(&self, pos: Pos) -> Rect {
        Rect {
            left: self.left + pos.x,
            top: self.top + pos.y,
            width: self.width,
            height: self.height
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect {} -> {}", self.origin, self.size)
    }
}

pub trait Scalable {
    fn scale(&self, factor: f64) -> Self;
}

impl Scalable for Pos {
    fn scale(&self, factor: f64) -> Pos {
        Pos {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Scalable for Rect {
    fn scale(&self, factor: f64) -> Self {
        Rect {
            left: self.left * factor,
            top: self.top * factor,
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

impl Scalable for f64 {
    fn scale(&self, factor: f64) -> Self {
        *self * factor
    }
}

impl Scalable for i32 {
    fn scale(&self, factor: f64) -> Self {
        let temp = (*self as f64) * factor;
        temp.try_into().unwrap()
    }
}

impl Scalable for Size {
    fn scale(&self, factor: f64) -> Self {
        Size {
            height: self.height * factor,
            width: self.width * factor
        }
    }
}