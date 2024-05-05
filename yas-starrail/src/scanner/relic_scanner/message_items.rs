use image::RgbImage;

pub struct SendItem {
    pub panel_image: RgbImage,
    pub equip: String,
    pub star: usize,
    pub lock: bool,
    pub discard: bool,
}