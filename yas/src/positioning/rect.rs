use std::fmt::Display;
use std::ops::Add;
use serde::{Deserialize, Serialize};
use crate::positioning::{Pos, Scalable, Size};
use paste::paste;

#[derive(Debug, Clone, PartialEq, Default, Copy, Serialize, Deserialize)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T> where T: Copy {
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect {
            left, top, width, height
        }
    }

    pub fn origin(&self) -> Pos<T> {
        Pos {
            x: self.left,
            y: self.top
        }
    }

    pub fn size(&self) -> Size<T> {
        Size {
            width: self.width,
            height: self.height
        }
    }
}

impl<T> Rect<T> where T: Add<T, Output = T> + Copy {
    pub fn translate(&self, pos: Pos<T>) -> Rect<T> {
        Rect {
            left: self.left + pos.x,
            top: self.top + pos.y,
            width: self.width,
            height: self.height
        }
    }
}

impl<T> Display for Rect<T> where T: Display + Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect {} -> {}", self.origin(), self.size())
    }
}

impl Scalable for Rect<f64> {
    fn scale(&self, factor: f64) -> Self {
        Rect {
            left: self.left * factor,
            top: self.top * factor,
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

macro_rules! convert_rect_type {
    ($t1:ty, $t2:ty) => {
        impl Rect<$t1> {
            paste!{
                pub fn [<to_rect_ $t2>](&self) -> Rect<$t2> {
                    Rect {
                        left: self.left as $t2,
                        top: self.top as $t2,
                        width: self.width as $t2,
                        height: self.height as $t2,
                    }
                }
            }
        }
    }
}

convert_rect_type!(f64, i32);
convert_rect_type!(f64, usize);
convert_rect_type!(f64, u32);
convert_rect_type!(u32, usize);
convert_rect_type!(i32, usize);
convert_rect_type!(i32, f64);
convert_rect_type!(i32, u32);
convert_rect_type!(usize, i32);
