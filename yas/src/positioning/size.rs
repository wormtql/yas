use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::positioning::Scalable;

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy, Serialize, Deserialize)]
pub struct Size<T> {
    pub height: T,
    pub width: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Size<T> {
        Size {
            width, height
        }
    }
}

impl<T> Display for Size<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size({}, {})", self.height, self.width)
    }
}

macro impl_int_size($t:ty) {
    impl Scalable for Size<$t> {
        fn scale(&self, factor: f64) -> Self {
            Size {
                height: ((self.height as f64) * factor) as $t,
                width: ((self.width as f64) * factor) as $t,
            }
        }
    }
}

impl Scalable for Size<f64> {
    fn scale(&self, factor: f64) -> Self {
        Size {
            height: self.height * factor,
            width: self.width * factor
        }
    }
}

impl_int_size!(i32);
impl_int_size!(usize);
impl_int_size!(u32);

macro impl_int_hash($t:ty) {
    impl Hash for Size<$t> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.width.hash(state);
            self.height.hash(state);
        }
    }
}

impl_int_hash!(i32);
impl_int_hash!(usize);
impl_int_hash!(u32);
