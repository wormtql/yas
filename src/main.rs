use std::time::{Duration, Instant, SystemTime};
use std::io::stdin;

use yas::common::utils;
use yas::capture::{capture_absolute, capture_absolute_image};
use yas::inference::pre_process::{to_gray, raw_to_img, normalize, crop, pre_process, image_to_raw};
use yas::info::info;

use winapi::um::winuser::{SetForegroundWindow};

use image::{ImageBuffer, Pixel};
use image::imageops::grayscale;
use yas::common::{RawImage, PixelRect};
use yas::scanner::yas_scanner::YasScanner;
use yas::inference::inference::CRNNModel;
use yas::expo::mona_uranai::MonaFormat;
use env_logger::{Env, Builder, Target};
use log::{info, LevelFilter};

fn open_local(path: String) -> RawImage {
    let img = image::open(path).unwrap();
    let img = grayscale(&img);
    let raw_img = image_to_raw(img);

    raw_img
}

fn main() {
    // let mut raw = open_local(String::from("data/test/15.png"));
    // let raw = pre_process(raw);
    // // normalize(&mut raw, true);
    // let img = raw_to_img(&raw);
    // img.save("test.png").unwrap();

    Builder::new().filter_level(LevelFilter::Info).init();

    if !utils::is_admin() {
        utils::error_and_quit("请以管理员身份运行该程序")
    }

    let hwnd = match utils::find_window(String::from("原神")) {
        Err(s) => {
            utils::error_and_quit("未找到原神窗口，请确认原神已经开启");
        },
        Ok(h) => h,
    };

    unsafe { SetForegroundWindow(hwnd); }
    utils::sleep(1000);

    let rect = utils::get_client_rect(hwnd).unwrap();

    let mut info: info::ScanInfo;
    if rect.height * 16 == rect.width * 9 {
        info = info::ScanInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left as u32, rect.top as u32);
    } else if rect.height * 8 == rect.width * 5 {
        info = info::ScanInfo::from_8_5(rect.width as u32, rect.height as u32, rect.left as u32, rect.top as u32);
    } else if rect.height * 4 == rect.width * 3 {
        info = info::ScanInfo::from_4_3(rect.width as u32, rect.height as u32, rect.left as u32, rect.top as u32);
    } else {
        utils::error_and_quit("不支持的分辨率");
    }

    let mut scanner = YasScanner::new(info.clone());

    let now = SystemTime::now();
    let results = scanner.start();
    let mona = MonaFormat::new(&results);
    let t = now.elapsed().unwrap().as_secs_f64();
    info!("time: {}s", t);
    mona.save(String::from("mona.json"));
    // let info = info;
    // let img = info.art_count_position.capture_relative(&info).unwrap();

    // let mut inference = CRNNModel::new(String::from("model_training.onnx"), String::from("index_2_word.json"));
    // let s = inference.inference_string(&img);
    // println!("{}", s);
    info!("识别结束，请按Enter退出");
    let mut s = String::new();
    stdin().read_line(&mut s);
}
