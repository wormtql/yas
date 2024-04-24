mod traits;
mod yas_model;
mod paddle_paddle_model;

pub use yas_model::yas_ocr_model::YasOCRModel;
pub use yas_model::yas_ocr_model::yas_ocr_model;
pub use traits::ImageToText;
pub use paddle_paddle_model::PPOCRModel;
pub use paddle_paddle_model::PPOCRChV4RecInfer;
pub use paddle_paddle_model::ppocr_model;
