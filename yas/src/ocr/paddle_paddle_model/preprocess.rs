use image::{ImageBuffer, RgbImage};
use crate::positioning::Shape3D;
use anyhow::Result;
use image::imageops::{FilterType, resize};
use tract_onnx::prelude::{Tensor, tract_ndarray};

/// Resize an image to the expected height, but the width can vary
/// rec_image_shape: the expected shape to feed into the onnx model. CHW
pub fn resize_img(rec_image_shape: Shape3D<u32>, img: &RgbImage) -> RgbImage {
    let image_width = img.width();
    let image_height = img.height();
    let wh_ratio = image_width as f64 / image_height as f64;

    assert_eq!(rec_image_shape.x, 3);

    let resized_width = (wh_ratio * rec_image_shape.y as f64) as u32;

    let resized_image = resize(img, resized_width, rec_image_shape.y, FilterType::Triangle);
    resized_image
}

pub fn normalize_image_to_tensor(img: &RgbImage) -> Tensor {
    let height = img.height() as usize;
    let width = img.width() as usize;
    let tensor: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, height, width), |(_, c, y, x)| {
        let pix = img.get_pixel(x as u32, y as u32)[c];
        let v = pix as f32 / 255.0_f32;
        (v - 0.5) / 0.5
    }).into();
    tensor
}
