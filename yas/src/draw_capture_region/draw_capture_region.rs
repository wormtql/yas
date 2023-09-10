use crate::positioning::{Pos, Rect};

pub trait DrawCaptureRegion {
    fn draw_capture_region(&self, image: &mut image::RgbImage);
}

impl<T> DrawCaptureRegion for Pos<T> where T: TryInto<u32> {
    fn draw_capture_region(&self, image: &mut image::RgbImage) {
        let blue = image::Rgb([0, 0, 255]);
        let x = 1.0;

        let x: u32 = self.x.as_primitive();
        let y: u32 = self.y.as_primitive();

        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                image.put_pixel(i, j, blue);
            }
        }

        for i in x - 5..=x + 5 {
            image.put_pixel(i, y + 5, blue);
            image.put_pixel(i, y - 5, blue);
        }

        for j in y - 5..=y + 5 {
            image.put_pixel(x + 5, j, blue);
            image.put_pixel(x - 5, j, blue);
        }
    }
}

impl<T> DrawCaptureRegion for Rect<T> where T: TryInto<u32> {
    fn draw_capture_region(&self, image: &mut image::RgbImage) {
        let red = image::Rgb([255, 0, 0]);

        let left: u32 = self.left.as_primitive();
        let top: u32 = self.top.as_primitive();
        let width: u32 = self.width.as_primitive();
        let height: u32 = self.height.as_primitive();
        let bottom = top + height;
        let right = left + width;

        for x in left..right {
            image.put_pixel(x, top, red);
            image.put_pixel(x, bottom, red);
        }

        for y in top..bottom {
            image.put_pixel(left, y, red);
            image.put_pixel(right, y, red);
        }
    }
}

// impl DrawConfig for SharedScanInfo {
//     fn draw_capture_region(&self, image: &mut image::RgbImage) {
//         self.title_pos.draw_config(image);
//         self.main_stat_name_pos.draw_config(image);
//         self.main_stat_value_pos.draw_config(image);
//
//         self.level_pos.draw_config(image);
//         self.panel_pos.draw_config(image);
//
//         self.item_equip_pos.draw_config(image);
//         self.item_count_pos.draw_config(image);
//
//         self.pool_pos.draw_config(image);
//
//         self.flag.draw_config(image);
//         self.star.draw_config(image);
//     }
// }
