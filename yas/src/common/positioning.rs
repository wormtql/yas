use std::ops::{Add, Mul, Sub};
use std::fmt::Display;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Size<T> {
    pub height: T,
    pub width: T,
}

impl<T> Add<Pos<T>> for Pos<T> where T: Add<T> {
    type Output = Self;

    fn add(self, rhs: Pos<T>) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Pos<T>> for Pos<T> where T: Sub<T> {
    type Output = Self;

    fn sub(self, rhs: Pos<T>) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Rect<T> {
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect {
            left, top, width, height
        }
    }
}

impl<T> Pos<T> {
    pub fn new(x: T, y: T) -> Pos<T> {
        Pos {
            x, y
        }
    }
}


impl<T> Rect<T> where T: Copy + Add<T> {
    pub fn translate(&self, pos: Pos<T>) -> Rect<T> {
        Rect {
            left: self.left + pos.x,
            top: self.top + pos.y,
            width: self.width,
            height: self.height
        }
    }
}

impl<T> Display for Pos<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Display for Rect<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect {} -> {}", self.origin, self.size)
    }
}


pub trait Scalable<T> {
    fn scale(&self, factor: T) -> Self;
}

impl<ScaleType, PosType> Scalable<ScaleType> for Pos<PosType>
where
    ScaleType: TryInto<PosType>,
    PosType: Copy + Mul<PosType>
{
    fn scale(&self, factor: ScaleType) -> Pos<PosType> {
        let factor: PosType = factor.try_into().unwrap();
        Pos {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl<ScaleType, RectType> Scalable<ScaleType> for Rect<RectType>
where
    ScaleType: TryInto<RectType>,
    RectType: Copy + Mul<RectType>
{
    fn scale(&self, factor: ScaleType) -> Self {
        let factor: RectType = factor.try_into().unwrap();
        Rect {
            left: self.left * factor,
            top: self.top * factor,
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}