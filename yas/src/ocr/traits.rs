use std::time::Duration;

use anyhow::Result;

pub trait ImageToText<ImageType> {
    fn image_to_text(&self, image: &ImageType, is_preprocessed: bool) -> Result<String>;

    fn get_average_inference_time(&self) -> Option<Duration>;
}

// pub trait ImageTextDetection<ImageType> {
//
// }
