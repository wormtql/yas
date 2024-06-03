use anyhow::Result;
use image::{RgbaImage, RgbImage};
use image::buffer::ConvertBuffer;
use libwayshot::{WayshotConnection, CaptureRegion};
use crate::capture::Capturer;
use crate::positioning::Rect;

pub struct LibwayshotCapturer {
    conn: WayshotConnection,
}

impl LibwayshotCapturer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            conn: WayshotConnection::new()?,
        })
    }
}

impl Capturer<RgbaImage> for LibwayshotCapturer {
    fn capture_rect(&self, rect: Rect<i32>) -> Result<RgbaImage> {
        let region = CaptureRegion {
            x_coordinate: rect.left,
            y_coordinate: rect.top,
            width: rect.width,
            height: rect.height,
        };
        let capture_result = self.conn.screenshot(region, false)?;
        Ok(capture_result)
    }
}

impl Capturer<RgbImage> for LibwayshotCapturer {
    fn capture_rect(&self, rect: Rect<i32>) -> Result<RgbImage> {
        let rgba_result: RgbaImage = self.capture_rect(rect)?;
        Ok(rgba_result.convert())
    }
}
