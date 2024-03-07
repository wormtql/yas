pub use config::GenshinRepositoryScannerLogicConfig;
pub use controller::GenshinRepositoryScanController;
pub use controller::ReturnResult;
pub use scroll_result::ScrollResult;
pub use window_info::GenshinRepositoryScanControllerWindowInfo;

mod config;
mod controller;

mod scroll_result;
mod window_info;

