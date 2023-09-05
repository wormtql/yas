use super::*;
use std::ops::{Add, AddAssign};

macro_rules! impl_add {
    ( $i:ty, $u:ty ) => {
        impl<'a, 'b> Add<&'b Pos<$i>> for &'a Pos<$i> {
            type Output = Pos<$i>;

            fn add(self, rhs: &Pos<$i>) -> Pos<$i> {
                Pos {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl<'a, 'b> Add<&'b Pos<$i>> for &'a Rect<$i, $u> {
            type Output = Rect<$i, $u>;

            fn add(self, rhs: &Pos<$i>) -> Rect<$i, $u> {
                Rect {
                    origin: &self.origin + rhs,
                    size: self.size,
                }
            }
        }

        impl<'a, 'b> Add<&'b Pos<$i>> for &'a RectBound<$i> {
            type Output = RectBound<$i>;

            fn add(self, rhs: &Pos<$i>) -> RectBound<$i> {
                RectBound {
                    left: self.left + rhs.x,
                    top: self.top + rhs.y,
                    right: self.right + rhs.x,
                    bottom: self.bottom + rhs.y,
                }
            }
        }

        impl ops::AddAssign<Pos<$i>> for Pos<$i> {
            fn add_assign(&mut self, rhs: Pos<$i>) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl ops::AddAssign<Pos<$i>> for Rect<$i, $u> {
            fn add_assign(&mut self, rhs: Pos<$i>) {
                self.origin += rhs;
            }
        }

        impl ops::AddAssign<Pos<$i>> for RectBound<$i> {
            fn add_assign(&mut self, rhs: Pos<$i>) {
                self.left += rhs.x;
                self.top += rhs.y;
                self.right += rhs.x;
                self.bottom += rhs.y;
            }
        }
    };
}

impl_add!(i32, u32);
impl_add!(f32, f32);
impl_add!(f64, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i32() {
        let pos1 = Pos::new(1, 2);
        let pos2 = Pos::new(3, 4);
        let pos3 = Pos::new(4, 6);

        assert_eq!(&pos1 + &pos2, pos3);
    }

    #[test]
    fn test_f64() {
        let pos1 = Pos::new(1.0, 2.5);
        let pos2 = Pos::new(3.3, 4.0);
        let pos3 = Pos::new(4.3, 6.5);

        assert_eq!(&pos1 + &pos2, pos3);
    }
}
