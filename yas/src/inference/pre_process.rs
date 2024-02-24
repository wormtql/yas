use image::imageops::{overlay, resize};
use image::{GenericImageView, GrayImage, ImageBuffer, Luma, RgbImage};

pub type GrayImageFloat = ImageBuffer<Luma<f32>, Vec<f32>>;

pub trait ImageConvExt {
    fn to_common_grayscale(&self) -> GrayImage;
}

impl ImageConvExt for GrayImageFloat {
    fn to_common_grayscale(&self) -> GrayImage {
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            let pv = self.get_pixel(x, y)[0];
            let pixel = (pv * 255.0) as u32;
            let pixel: u8 = if pixel > 255 { 255 } else { pixel as _ };
            image::Luma([pixel])
        })
    }
}
