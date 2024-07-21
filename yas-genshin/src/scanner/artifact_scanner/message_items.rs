use image::RgbImage;

/// this is constructed by the capturing thread, and sent to the worker thread
pub struct SendItem {
    pub panel_image: RgbImage,
    pub star: usize,
    pub game_image: Option<RgbImage>,
}
