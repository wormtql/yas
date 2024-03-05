use std::fmt::Display;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::positioning::Scalable;

#[derive(Debug, Clone, PartialEq, Default, Copy, Serialize, Deserialize)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add<Pos<T>> for Pos<T> where T: Add<T, Output = T> {
    type Output = Self;

    fn add(self, rhs: Pos<T>) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Pos<T>> for Pos<T> where T: Sub<T, Output = T> {
    type Output = Self;

    fn sub(self, rhs: Pos<T>) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

impl<T> Display for Pos<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Scalable for Pos<f64> {
    fn scale(&self, factor: f64) -> Pos<f64> {
        Pos {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

macro impl_int_pos($t:ty) {
    impl Scalable for Pos<$t> {
        fn scale(&self, factor: f64) -> Pos<$t> {
            Pos {
                x: ((self.x as f64) * factor) as $t,
                y: ((self.y as f64) * factor) as $t
            }
        }
    }
}

impl_int_pos!(i32);
impl_int_pos!(usize);
impl_int_pos!(u32);
