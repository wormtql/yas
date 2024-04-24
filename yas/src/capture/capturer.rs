use crate::positioning::{Pos, Rect};
use anyhow::Result;

pub trait Capturer<T: image::GenericImage> {
    // it's necessary to use signed int, because capture region may be out of the screen
    fn capture_rect(&self, rect: Rect<i32>) -> Result<T>;

    fn capture_color(&self, pos: Pos<i32>) -> Result<T::Pixel> {
        let image = self.capture_rect(Rect {
            left: pos.x,
            top: pos.y,
            width: 1,
            height: 1,
        })?;
        Ok(image.get_pixel(0, 0))
    }

    fn capture_relative_to(&self, rect: Rect<i32>, relative_to: Pos<i32>) -> Result<T> {
        let new_rect = Rect {
            left: rect.left + relative_to.x,
            top: rect.top + relative_to.y,
            width: rect.width,
            height: rect.height
        };
        self.capture_rect(new_rect)
    }
}
