use crate::capture::{Capturer, ScreenshotsCapturer};
#[cfg(target_os = "windows")]
use crate::capture::WinapiCapturer;
use anyhow::Result;
use image::RgbImage;
use crate::positioning::Rect;

pub struct GenericCapturer {
    #[cfg(target_os = "windows")]
    pub windows_capturer: WinapiCapturer,
    pub fallback_capturer: ScreenshotsCapturer,
}

impl GenericCapturer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            #[cfg(target_os = "windows")]
            windows_capturer: WinapiCapturer::new(),
            fallback_capturer: ScreenshotsCapturer::new()?,
        })
    }
}

impl Capturer<RgbImage> for GenericCapturer {
    fn capture_rect(&self, rect: Rect<i32>) -> Result<RgbImage> {
        #[cfg(target_os = "windows")]
        {
            let result = self.windows_capturer.capture_rect(rect);
            if result.is_ok() {
                return result
            }
        }

        let result = self.fallback_capturer.capture_rect(rect);
        result
    }
}
