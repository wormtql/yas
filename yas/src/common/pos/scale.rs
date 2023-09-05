use super::*;

pub trait Scalable {
    fn scale(&mut self, ratio: f64);
}

impl Scalable for i32 {
    fn scale(&mut self, ratio: f64) {
        *self = (*self as f64 * ratio).round() as i32;
    }
}

impl Scalable for u32 {
    fn scale(&mut self, ratio: f64) {
        *self = (*self as f64 * ratio).round() as u32;
    }
}

impl Scalable for f32 {
    fn scale(&mut self, ratio: f64) {
        *self = (*self as f64 * ratio) as f32;
    }
}

impl Scalable for f64 {
    fn scale(&mut self, ratio: f64) {
        *self *= ratio;
    }
}

impl<T> Scalable for Size<T>
where
    T: Scalable,
{
    fn scale(&mut self, ratio: f64) {
        self.width.scale(ratio);
        self.height.scale(ratio);
    }
}

impl<T> Scalable for Pos<T>
where
    T: Scalable,
{
    fn scale(&mut self, ratio: f64) {
        self.x.scale(ratio);
        self.y.scale(ratio);
    }
}

impl<P, S> Scalable for Rect<P, S>
where
    P: Scalable,
    S: Scalable,
{
    fn scale(&mut self, ratio: f64) {
        self.origin.scale(ratio);
        self.size.scale(ratio);
    }
}

impl<T> Scalable for RectBound<T>
where
    T: Scalable,
{
    fn scale(&mut self, ratio: f64) {
        self.left.scale(ratio);
        self.top.scale(ratio);
        self.right.scale(ratio);
        self.bottom.scale(ratio);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale() {
        let mut pos = Pos::new(100, 200);
        pos.scale(0.5);
        assert_eq!(pos, Pos::new(50, 100));

        let mut size = Size::new(100, 200);
        size.scale(0.5);
        assert_eq!(size, Size::new(50, 100));

        let mut rect = Rect::new(100, 200, 300, 400);
        rect.scale(0.5);
        assert_eq!(rect, Rect::new(50, 100, 150, 200));

        let mut bound = RectBound::new(100, 200, 300, 400);
        bound.scale(0.5);
        assert_eq!(bound, RectBound::new(50, 100, 150, 200));
    }

    #[test]
    fn test_float_scale() {
        let mut pos = Pos::new(100.0, 200.0);
        pos.scale(0.5);
        assert_eq!(pos, Pos::new(50.0, 100.0));

        let mut size = Size::new(100.0, 200.0);
        size.scale(0.5);
        assert_eq!(size, Size::new(50.0, 100.0));

        let mut rect = Rect::new(100.0, 200.0, 300.0, 400.0);
        rect.scale(0.5);
        assert_eq!(rect, Rect::new(50.0, 100.0, 150.0, 200.0));

        let mut bound = RectBound::new(100.0, 200.0, 300.0, 400.0);
        bound.scale(0.5);
        assert_eq!(bound, RectBound::new(50.0, 100.0, 150.0, 200.0));
    }
}
