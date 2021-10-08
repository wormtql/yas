use std::time::{Duration, Instant, SystemTime};
use std::path::Path;
use std::io::stdin;

use yas::common::utils;
use yas::capture::{capture_absolute, capture_absolute_image};
use yas::inference::pre_process::{to_gray, raw_to_img, normalize, crop, pre_process, image_to_raw};
use yas::info::info;

use winapi::um::winuser::{SetForegroundWindow, GetDpiForSystem, SetThreadDpiAwarenessContext, ShowWindow, SW_SHOW, SW_RESTORE};

use clap::{Arg, App};

use image::{ImageBuffer, Pixel};
use image::imageops::grayscale;
use yas::common::{RawImage, PixelRect};
use yas::scanner::yas_scanner::{YasScanner, YasScannerConfig};
use yas::inference::inference::CRNNModel;
use yas::expo::mona_uranai::MonaFormat;
use env_logger::{Env, Builder, Target};
use log::{info, LevelFilter};
use winapi::shared::windef::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE;

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

    let matches = App::new("YAS - 原神圣遗物导出器")
        .version("0.1.0")
        .author("wormtql <584130248@qq.com>")
        .about("Genshin Impact Artifact Exporter")
        .arg(Arg::with_name("max-row").long("max-row").takes_value(true).help("最大扫描行数"))
        .arg(Arg::with_name("capture-only").long("capture-only").required(false).takes_value(false).help("只保存截图，不进行扫描，debug专用"))
        .arg(Arg::with_name("min-star").long("min-star").takes_value(true).help("最小星级").min_values(1).max_values(5))
        .arg(Arg::with_name("max-wait-switch-artifact").long("max-wait-switch-artifact").takes_value(true).min_values(10).help("切换圣遗物最大等待时间(ms)"))
        .arg(Arg::with_name("output-dir").long("output-dir").short("o").takes_value(true).help("输出目录").default_value("."))
        .get_matches();
    let config = YasScannerConfig::from_match(&matches);

    unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE); }
    let hwnd = match utils::find_window(String::from("原神")) {
        Err(s) => {
            utils::error_and_quit("未找到原神窗口，请确认原神已经开启");
        },
        Ok(h) => h,
    };

    unsafe { ShowWindow(hwnd, SW_RESTORE); }
    // utils::sleep(1000);
    unsafe { SetForegroundWindow(hwnd); }
    utils::sleep(1000);

    let mut rect = utils::get_client_rect(hwnd).unwrap();

    // rect.scale(1.25);
    info!("detected left: {}", rect.left);
    info!("detected top: {}", rect.top);
    info!("detected width: {}", rect.width);
    info!("detected height: {}", rect.height);

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

    let mut scanner = YasScanner::new(info.clone(), config);

    let now = SystemTime::now();
    let results = scanner.start();
    let mona = MonaFormat::new(&results);
    let t = now.elapsed().unwrap().as_secs_f64();
    info!("time: {}s", t);

    let output_dir = Path::new(matches.value_of("output-dir").unwrap());
    let output_filename = output_dir.join("mona.json");
    mona.save(String::from(output_filename.to_str().unwrap()));
    // let info = info;
    // let img = info.art_count_position.capture_relative(&info).unwrap();

    // let mut inference = CRNNModel::new(String::from("model_training.onnx"), String::from("index_2_word.json"));
    // let s = inference.inference_string(&img);
    // println!("{}", s);
    info!("识别结束，请按Enter退出");
    let mut s = String::new();
    stdin().read_line(&mut s);
}
