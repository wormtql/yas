use image::{ImageBuffer, Luma, RgbImage, GenericImageView};
use image::imageops;

/// convert rgb image to f32 gray image
pub fn to_gray(raw: &RgbImage) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let mut new_gray: ImageBuffer<Luma<f32>, Vec<f32>> = ImageBuffer::new(raw.width(), raw.height());
    for x in 0..raw.width() {
        for y in 0..raw.height() {
            let rgb = raw.get_pixel(x, y);

            let r = rgb[0] as f32 / 255.0;
            let g = rgb[1] as f32 / 255.0;
            let b = rgb[2] as f32 / 255.0;

            let gray = r * 0.2989 + g * 0.5870 + b * 0.1140;
            let grayp = new_gray.get_pixel_mut(x, y);
            grayp[0] = gray;
        }
    }
    new_gray
}

/// normalize an f32 gray image
/// which makes the bright pixel more bright, the dark pixels more dark
fn normalize(im: &mut ImageBuffer<Luma<f32>, Vec<f32>>, auto_inverse: bool) -> bool {
    let width = im.width();
    let height = im.height();

    if width == 0 || height == 0 {
        println!("wrong width or height");
        return false;
    }

    let mut max: f32 = 0.0;
    let mut min: f32 = 256.0;

    for i in 0..width {
        for j in 0..height {
            let p = im.get_pixel(i, j)[0];
            if p > max {
                max = p;
            }
            if p < min {
                min = p;
            }
        }
    }

    if max == min {
        return false;
    }

    let flag_pixel = im.get_pixel(width - 2, height - 1)[0];
    let flag_pixel = (flag_pixel - min) / (max - min);

    for i in 0..width {
        for j in 0..height {
            let p = im.get_pixel_mut(i, j);
            let pv = p[0];
            let mut new_pv = (pv - min) / (max - min);
            if auto_inverse && flag_pixel > 0.5 {
                new_pv = 1.0 - new_pv;
            }
            p[0] = new_pv;
        }
    }

    true
}

/// crop an f32 gray image to only where there is text
fn crop(im: &ImageBuffer<Luma<f32>, Vec<f32>>) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let width = im.width();
    let height = im.height();

    let mut min_col = width - 1;
    let mut max_col = 0;
    let mut min_row = height - 1;
    let mut max_row = 0_u32;

    for i in 0..width {
        for j in 0..height {
            let p = im.get_pixel(i, j)[0];
            if p > 0.7 {
                if i < min_col {
                    min_col = i;
                }
                if i > max_col {
                    max_col = i;
                }
                break;
            }
        }
    }

    for j in 0..height {
        for i in 0..width {
            let p = im.get_pixel(i, j)[0];
            if p > 0.7 {
                if j < min_row {
                    min_row = j;
                }
                if j > max_row {
                    max_row = j;
                }
                break;
            }
        }
    }

    let new_height = max_row - min_row + 1;
    let new_width = max_col - min_col + 1;

    let _ans: Vec<f32> = vec![0.0; (new_width * new_height) as usize];
    let cropped_im = im.view(min_col, min_row, new_width, new_height).to_image();

    cropped_im
}

/// resize an f32 gray image to 384 * 32, if not wide enough, then pad with background
fn resize_and_pad(im: &ImageBuffer<Luma<f32>, Vec<f32>>) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let w = im.width();
    let h = im.height();

    let new_width = if w as f64 / (h as f64) > 384.0 / 32.0 {
        384
    } else {
        std::cmp::min((32.0 / h as f64 * w as f64) as u32, 384)
    };

    let new_height = std::cmp::min((384.0 / w as f64 * h as f64) as u32, 32);

    let img = imageops::resize(
        im,
        new_width,
        new_height,
        image::imageops::FilterType::Triangle,
    );

    let data: Vec<f32> = vec![0.0; 32 * 384];
    let mut padded_im = ImageBuffer::from_vec(384, 32, data).unwrap();
    imageops::overlay(&mut padded_im, &img, 0, 0);
    padded_im
}

/// transform an f32 gray image to a preprocessed image
/// if the image has only one color, then return false, but this is not an error
pub fn pre_process(im: ImageBuffer<Luma<f32>, Vec<f32>>) -> (ImageBuffer<Luma<f32>, Vec<f32>>, bool) {
    let mut im = im;
    if !normalize(&mut im, true) {
        return (im, false);
    }
    let mut im = crop(&im);

    normalize(&mut im, false);

    let mut im = resize_and_pad(&im);

    for i in 0..im.width() {
        for j in 0..im.height() {
            let p = im.get_pixel_mut(i, j);
            let pv = p[0];
            if pv < 0.53 {
                p[0] = 0.0;
            } else {
                p[0] = 1.0;
            }
        }
    }

    (im, true)
}
