use image::{GrayImage, ImageBuffer, Luma};

pub trait ToF32GrayImage {
    fn to_f32_gray_image(&self) -> ImageBuffer<Luma<f32>, Vec<f32>>;
}

impl ToF32GrayImage for GrayImage {
    fn to_f32_gray_image(&self) -> ImageBuffer<Luma<f32>, Vec<f32>> {
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            let pv = self.get_pixel(x, y)[0];
            Luma([pv as f32 / 255.0_f32])
        })
    }
}
