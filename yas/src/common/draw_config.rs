use crate::core::*;

use super::pos::*;

pub trait DrawConfig {
    fn draw_config(&self, image: &mut image::RgbImage);
}

impl DrawConfig for Pos<ScanInfoType> {
    fn draw_config(&self, image: &mut image::RgbImage) {
        let blue = image::Rgb([0, 0, 255]);

        for i in self.x - 1..=self.x + 1 {
            for j in self.y - 1..=self.y + 1 {
                image.put_pixel(i, j, blue);
            }
        }

        for i in self.x - 5..=self.x + 5 {
            image.put_pixel(i, self.y + 5, blue);
            image.put_pixel(i, self.y - 5, blue);
        }

        for j in self.y - 5..=self.y + 5 {
            image.put_pixel(self.x + 5, j, blue);
            image.put_pixel(self.x - 5, j, blue);
        }
    }
}

impl DrawConfig for RectBound<ScanInfoType> {
    fn draw_config(&self, image: &mut image::RgbImage) {
        let red = image::Rgb([255, 0, 0]);

        for x in self.left..self.right {
            image.put_pixel(x, self.top, red);
            image.put_pixel(x, self.bottom, red);
        }

        for y in self.top..self.bottom {
            image.put_pixel(self.left, y, red);
            image.put_pixel(self.right, y, red);
        }
    }
}

impl DrawConfig for Rect<ScanInfoType, ScanInfoType> {
    fn draw_config(&self, image: &mut image::RgbImage) {
        RectBound::from(self).draw_config(image);
    }
}

impl DrawConfig for SharedScanInfo {
    fn draw_config(&self, image: &mut image::RgbImage) {
        self.title_pos.draw_config(image);
        self.main_stat_name_pos.draw_config(image);
        self.main_stat_value_pos.draw_config(image);

        self.level_pos.draw_config(image);
        self.panel_pos.draw_config(image);

        self.item_equip_pos.draw_config(image);
        self.item_count_pos.draw_config(image);

        self.pool_pos.draw_config(image);

        self.flag.draw_config(image);
        self.star.draw_config(image);
    }
}
