#![feature(decl_macro)]
#![feature(concat_idents)]
#![allow(unused_imports)]

#[cfg(all(feature = "ort", feature = "tract_onnx"))]
compile_error!("feature \"ort\" and \"tract_onnx\" cannot be enabled at the same time");

extern crate log;
extern crate lazy_static;

pub mod common;
pub mod export;
pub mod draw_capture_region;
pub mod capture;
pub mod utils;
pub mod game_info;
pub mod window_info;
pub mod system_control;
pub mod ocr;
pub mod positioning;
