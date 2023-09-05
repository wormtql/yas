use super::*;

/// Scale the position.
pub trait Scalable {
    fn scale(self, ratio: f64) -> Self;
}

impl Scalable for i32 {
    fn scale(self, ratio: f64) -> Self {
        (self as f64 * ratio).round() as i32
    }
}

impl Scalable for u32 {
    fn scale(self, ratio: f64) -> Self {
        (self as f64 * ratio).round() as u32
    }
}

impl Scalable for f32 {
    fn scale(self, ratio: f64) -> Self {
        (self as f64 * ratio) as f32
    }
}

impl Scalable for f64 {
    fn scale(self, ratio: f64) -> Self {
        self * ratio
    }
}

/// Scale the rectangle with width and height ratio.
pub trait RectScalable {
    fn rect_scale(self, width_ratio: f64, height_ratio: f64) -> Self;
}

impl<T> RectScalable for Pos<T>
where
    T: Scalable,
{
    fn rect_scale(self, width_ratio: f64, height_ratio: f64) -> Self {
        Self {
            x: self.x.scale(width_ratio),
            y: self.y.scale(height_ratio),
        }
    }
}

impl<T> Scalable for Pos<T>
where
    T: Scalable,
{
    fn scale(self, ratio: f64) -> Self {
        Self {
            x: self.x.scale(ratio),
            y: self.y.scale(ratio),
        }
    }
}

impl<T> RectScalable for Size<T>
where
    T: Scalable,
{
    fn rect_scale(self, width_ratio: f64, height_ratio: f64) -> Self {
        Self {
            width: self.width.scale(width_ratio),
            height: self.height.scale(height_ratio),
        }
    }
}

impl<T> Scalable for Size<T>
where
    T: Scalable,
{
    fn scale(self, ratio: f64) -> Self {
        self.rect_scale(ratio, ratio)
    }
}

impl<P, S> RectScalable for Rect<P, S>
where
    P: Scalable,
    S: Scalable,
{
    fn rect_scale(self, width_ratio: f64, height_ratio: f64) -> Self {
        Self {
            origin: self.origin.rect_scale(width_ratio, height_ratio),
            size: self.size.rect_scale(width_ratio, height_ratio),
        }
    }
}

impl<P, S> Scalable for Rect<P, S>
where
    P: Scalable,
    S: Scalable,
{
    fn scale(self, ratio: f64) -> Self {
        self.rect_scale(ratio, ratio)
    }
}

impl<T> RectScalable for RectBound<T>
where
    T: Scalable,
{
    fn rect_scale(self, width_ratio: f64, height_ratio: f64) -> Self {
        Self {
            left: self.left.scale(width_ratio),
            top: self.top.scale(height_ratio),
            right: self.right.scale(width_ratio),
            bottom: self.bottom.scale(height_ratio),
        }
    }
}

impl<T> Scalable for RectBound<T>
where
    T: Scalable,
{
    fn scale(self, ratio: f64) -> Self {
        self.rect_scale(ratio, ratio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale() {
        assert_eq!(Pos::new(100, 100).scale(0.5), Pos::new(50, 50));
        assert_eq!(Size::new(100, 100).scale(0.5), Size::new(50, 50));
        assert_eq!(
            Rect::new(100, 100, 100, 100).scale(0.5),
            Rect::new(50, 50, 50, 50)
        );
    }

    #[test]
    fn test_float_scale() {
        assert_eq!(Pos::new(100.0, 100.0).scale(0.5), Pos::new(50.0, 50.0));
        assert_eq!(Size::new(100.0, 100.0).scale(0.5), Size::new(50.0, 50.0));
        assert_eq!(
            Rect::new(100.0, 100.0, 100.0, 100.0).scale(0.5),
            Rect::new(50.0, 50.0, 50.0, 50.0)
        );
    }
}
