use std::time::{Duration, Instant, SystemTime};
use std::path::Path;
use std::io::stdin;

use yas::common::utils;
use yas::capture::{capture_absolute, capture_absolute_image};
use yas::inference::pre_process::{to_gray, raw_to_img, normalize, crop, pre_process, image_to_raw};
use yas::info::info;
use yas::common::{RawImage, PixelRect};
use yas::scanner::yas_scanner::{YasScanner, YasScannerConfig};
use yas::inference::inference::CRNNModel;
use yas::expo::mona_uranai::MonaFormat;

use winapi::um::winuser::{SetForegroundWindow, GetDpiForSystem, SetThreadDpiAwarenessContext, ShowWindow, SW_SHOW, SW_RESTORE, GetSystemMetrics, SetProcessDPIAware};
use winapi::shared::windef::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE;
use winapi::um::wingdi::{HORZRES, GetDeviceCaps};
use winapi::um::shellscalingapi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

use clap::{Arg, App};
use image::{ImageBuffer, Pixel};
use image::imageops::grayscale;
use env_logger::{Env, Builder, Target};
use log::{info, LevelFilter};
use os_info;

use yas::artifact::internal_artifact::{InternalArtifact};
use colored::*;
use yas::item::items::{GeneshinItem};

fn open_local(path: String) -> RawImage {
    let img = image::open(path).unwrap();
    let img = grayscale(&img);
    let raw_img = image_to_raw(img);

    raw_img
}

fn set_dpi_awareness() {
    let os = os_info::get();

    // unsafe  {
    //     SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    // }
    if os.version() >= &os_info::Version::from_string("8.1") {
        //info!("Windows version >= 8.1");
        info!("{} >= {}","Windows version".blue(),"8.1".green());
        //println!("{} {} !", "it".green(), "works".blue().bold());
        unsafe  {
            SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
        }
    } else {
        info!("Windows version < 8.1");
        unsafe {
            SetProcessDPIAware();
        }
    }
}

fn get_version() -> String {
    let s = include_str!("../Cargo.toml");
    for line in s.lines() {
        if line.starts_with("version = ") {
            let temp = line.split("\"").collect::<Vec<_>>();
            return String::from(temp[temp.len() - 2]);
        }
    }

    String::from("unknown_version")
}

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    if !utils::is_admin() {
        utils::error_and_quit(&"请以管理员身份运行该程序".red().bold())
    }

    let version = get_version();

    let matches = App::new("YAS TeyvatWikiEdit")
        .version(version.as_str())
        .author("wormtql <584130248@qq.com>(origin author) | Lounode <lounode@gmail.com>(editor)")
        .about("Genshin Impact Artifact Exporter")
        .arg(Arg::with_name("max-row").long("max-row").takes_value(true).help("最大扫描行数"))
        .arg(Arg::with_name("dump").long("dump").required(false).takes_value(false).help("输出模型预测结果、二值化图像和灰度图像，debug专用"))
        .arg(Arg::with_name("capture-only").long("capture-only").required(false).takes_value(false).help("只保存截图，不进行扫描，debug专用"))
        .arg(Arg::with_name("min-star").long("min-star").takes_value(true).help("最小星级").min_values(1).max_values(5))
        .arg(Arg::with_name("max-wait-switch-artifact").long("max-wait-switch-artifact").takes_value(true).min_values(10).help("切换圣遗物最大等待时间(ms)"))
        .arg(Arg::with_name("output-dir").long("output-dir").short("o").takes_value(true).help("输出目录").default_value("."))
        .arg(Arg::with_name("scroll-stop").long("scroll-stop").takes_value(true).help("翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项，默认为80）"))
        .arg(Arg::with_name("number").long("number").takes_value(true).help("指定圣遗物数量（在自动识别数量不准确时使用）").min_values(1).max_values(1500))
        .arg(Arg::with_name("verbose").long("verbose").help("显示详细信息"))
        .arg(Arg::with_name("offset-x").long("offset-x").takes_value(true).help("人为指定横坐标偏移（截图有偏移时可用该选项校正）"))
        .arg(Arg::with_name("offset-y").long("offset-y").takes_value(true).help("人为指定纵坐标偏移（截图有偏移时可用该选项校正）"))
        .get_matches();
    let config = YasScannerConfig::from_match(&matches);

    set_dpi_awareness();
    
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

    let rect = utils::get_client_rect(hwnd).unwrap();

    // rect.scale(1.25);
    info!("detected left: {}", rect.left);
    info!("detected top: {}", rect.top);
    info!("detected width: {}", rect.width);
    info!("detected height: {}", rect.height);
    
    let temp = capture_absolute_image(&rect).unwrap().save("test.png");

    let mut info: info::ScanInfo;
    if rect.height * 16 == rect.width * 9 {
        info = info::ScanInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else if rect.height * 8 == rect.width * 5 {
        info = info::ScanInfo::from_8_5(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else if rect.height * 4 == rect.width * 3 {
        info = info::ScanInfo::from_4_3(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else {
        utils::error_and_quit("不支持的分辨率");
    }

    let offset_x = matches.value_of("offset-x").unwrap_or("0").parse::<i32>().unwrap();
    let offset_y = matches.value_of("offset-y").unwrap_or("0").parse::<i32>().unwrap();
    info.left += offset_x;
    info.top += offset_y;
    
    //ScanData
        //Artifact
    fn scanAtifact (info: info::ScanInfo, config: YasScannerConfig) -> Vec<InternalArtifact> {
        let mut scanner = YasScanner::new(info.clone(), config);
        let now = SystemTime::now();
        let results = scanner.start();
        let t = now.elapsed().unwrap().as_secs_f64().to_string();
        info!("scan successfully. time: {}s", t.green());
        results
    }
        //Item
    fn scanItems (info: info::ScanInfo, config: YasScannerConfig) {//-> Vec<GeneshinItem> {
        info!("{}","Error! Scan items is already WIP.".red());
    }
    //OutPut
    fn outputArtifact (matches: clap::ArgMatches, results: Vec<InternalArtifact>) {
        info!("start ouput data, mod = {}","Artifact".green() );
        let now = SystemTime::now();
        let mona = MonaFormat::new(&results);
        let output_dir = Path::new(matches.value_of("output-dir").unwrap());
        let output_filename = output_dir.join("mona.json");
        mona.save(String::from(output_filename.to_str().unwrap()));
    }
    //Main Do
    let mut scanMode = "Artifact";
    info!("chose mode: {}", scanMode.red());
    let now = SystemTime::now();

    let mut artifactResult = scanAtifact(info, config);
    outputArtifact(matches, artifactResult);



    
    let allTime = now.elapsed().unwrap().as_secs_f64().to_string();
    info!("Total time: {}s", allTime.green());
    // let info = info;
    // let img = info.art_count_position.capture_relative(&info).unwrap();

    // let mut inference = CRNNModel::new(String::from("model_training.onnx"), String::from("index_2_word.json"));
    // let s = inference.inference_string(&img);
    // println!("{}", s);
    info!("识别结束，请按Enter退出");
    let mut s = String::new();
    stdin().read_line(&mut s);
}
