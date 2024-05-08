use anyhow::Result;
use image::RgbImage;
use yas::ocr::{ImageToText, PPOCRChV4RecInfer};
use image::io::Reader as ImageReader;

fn main() -> Result<()> {
    let model: Box<dyn ImageToText<RgbImage>> = Box::new(PPOCRChV4RecInfer::new()?);
    let image = ImageReader::open(r"E:\rust\yas\item_count.png")?.decode()?;
    let rgb_image = image.to_rgb8();
    let result = model.image_to_text(&rgb_image, false)?;
    println!("{}", result);

    Ok(())
}
