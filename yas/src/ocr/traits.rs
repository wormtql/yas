use anyhow::Result;

pub trait ImageToText<ImageType> {
    fn image_to_text(&self, image: &ImageType, is_preprocessed: bool) -> Result<String>;
}

// pub trait ImageTextDetection<ImageType> {
//
// }
