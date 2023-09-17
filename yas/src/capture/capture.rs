use crate::common::{color::Color, *};
use image::{buffer::ConvertBuffer, imageops::resize, imageops::FilterType::Triangle, RgbImage};

use anyhow::Result;
use crate::positioning::{Pos, Rect};

pub trait Capturable<ResultType> {
    fn capture(&self) -> Result<ResultType>;
}

pub trait RelativeCapturable<ResultType, PosType> where PosType: Copy + TryInto<i32> {
    fn capture_relative(&self, offset: Pos<PosType>) -> Result<ResultType>;
}

impl<ResultType, T, U> RelativeCapturable<ResultType, T> for Rect<U>
where
    T: Copy + TryInto<i32>,
    U: Copy + TryInto<i32>,
{
    fn capture_relative(&self, offset: Pos<T>) -> Result<ResultType> {
        (self + offset).capture()
    }
}

impl<T> Capturable<RgbImage> for Rect<T>
where
    T: Copy + TryInto<i32>
{
    fn capture(&self) -> Result<RgbImage> {
        // todo optimize screen logic
        let screen = screenshots::Screen::all()?[0];

        let left = self.left.try_into()?;
        let top = self.top.try_into()?;
        let width = self.width.try_into()?;
        let height = self.height.try_into()?;

        let mut rgb_img: RgbImage = screen
            .capture_area(
                left,
                top,
                width,
                height,
            )?
            .convert();

        if rgb_img.width() as i32 > width && rgb_img.height() as i32 > height {
            rgb_img = resize(&rgb_img, self.size.width, self.size.height, Triangle);
        }

        Ok(rgb_img)
    }
}

// impl<T> Capturable<GrayImageFloat> for Rect<T>
// where
//     T: Copy + TryInto<i32>
// {
//     fn capture(&self) -> Result<GrayImageFloat> {
//         let rgb_img: RgbImage = self.capture()?;

//         // todo refactor pre process logic
//         match pre_process(to_gray(&rgb_img)) {
//             Some(im) => Ok(im),
//             None => Err(anyhow::anyhow!("Capture error")),
//         }
//     }
// }

pub fn get_color(pos: Pos<i32>) -> Result<Color> {
    let rect: Rect<i32> = Rect {
        left: pos.x,
        top: pos.y,
        width: 1,
        height: 1,
    };
    let image: RgbImage = rect.capture()?;

    let pixel = image.get_pixel(0, 0);
    Ok(Color::new(pixel[0], pixel[1], pixel[2]))
}


