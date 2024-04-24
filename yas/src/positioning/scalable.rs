pub trait Scalable {
    fn scale(&self, factor: f64) -> Self;
}

impl Scalable for f64 {
    fn scale(&self, factor: f64) -> Self {
        *self * factor
    }
}

macro impl_int_scale($t:ty) {
    impl Scalable for $t {
        fn scale(&self, factor: f64) -> Self {
            ((*self as f64) * factor) as $t
        }
    }
}

impl_int_scale!(i32);
impl_int_scale!(usize);
impl_int_scale!(u32);
