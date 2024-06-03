use std::cell::RefCell;
use crate::capture::Capturer;
#[cfg(feature="capturer_screenshots")]
use crate::capture::ScreenshotsCapturer;
#[cfg(feature="capturer_libwayshot")]
use crate::capture::libwayshot_capturer::LibwayshotCapturer;
#[cfg(target_os = "windows")]
use crate::capture::WinapiCapturer;
use anyhow::{Result, anyhow};
use image::RgbImage;
use crate::positioning::Rect;

pub struct GenericCapturer {
    #[cfg(target_os = "windows")]
    windows_capturer: WinapiCapturer,
    #[cfg(feature="capturer_libwayshot")]
    libwayshot_capturer: RefCell<Option<LibwayshotCapturer>>,
    #[cfg(feature="capturer_screenshots")]
    fallback_capturer: ScreenshotsCapturer,
}

impl GenericCapturer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            #[cfg(target_os = "windows")]
            windows_capturer: WinapiCapturer::new(),
            #[cfg(feature="capturer_libwayshot")]
            libwayshot_capturer: RefCell::new(LibwayshotCapturer::new().ok()),
            #[cfg(feature="capturer_screenshots")]
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

        #[cfg(feature="capturer_libwayshot")]
        if self.libwayshot_capturer.borrow().is_some() {
            let result = self.libwayshot_capturer.borrow().as_ref().unwrap().capture_rect(rect);
            if result.is_err() {
              self.libwayshot_capturer.borrow_mut().take();
            } else {
              return result;
            }
        }

        #[cfg(feature="capturer_screenshots")]
        {
            let result = self.fallback_capturer.capture_rect(rect);
            return result;
        }

        Err(anyhow!("no enabled capturer!"))
    }
}
