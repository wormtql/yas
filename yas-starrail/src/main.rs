use std::io::stdin;
use std::path::Path;
use std::time::SystemTime;

#[cfg(target_os = "macos")]
use yas::common::utils::get_pid_and_ui;
use yas::common::{utils, UI};
use yas::common::{Rect, RawImage};
use yas::expo::march7th::March7thFormat;

use yas::inference::pre_process::image_to_raw;
use yas::core::starrail;
use yas::scanner::starrail::{YasScanner, YasScannerConfig};

use clap::{App, Arg};
use env_logger::Builder;
use image::imageops::grayscale;

use log::{info, warn, LevelFilter};

fn open_local(path: String) -> RawImage {
    let img = image::open(path).unwrap();
    let img = grayscale(&img);
    let raw_img = image_to_raw(img);

    raw_img
}

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    #[cfg(windows)]
    if !utils::is_admin() {
        utils::error_and_quit("请以管理员身份运行该程序")
    }

    if let Some(v) = utils::check_update() {
        warn!("检测到新版本，请手动更新：{}", v);
    }

    let matches = App::new("YAS - 崩坏：星穹铁道遗器导出器")
        .version(utils::VERSION)
        .author("wormtql <584130248@qq.com>")
        .about("Honkai: Star Rail Relic Exporter")
        .arg(
            Arg::with_name("max-row")
                .long("max-row")
                .takes_value(true)
                .help("最大扫描行数"),
        )
        .arg(
            Arg::with_name("dump")
                .long("dump")
                .required(false)
                .takes_value(false)
                .help("输出模型预测结果、二值化图像和灰度图像，debug专用"),
        )
        .arg(
            Arg::with_name("capture-only")
                .long("capture-only")
                .required(false)
                .takes_value(false)
                .help("只保存截图，不进行扫描，debug专用"),
        )
        .arg(
            Arg::with_name("min-star")
                .long("min-star")
                .takes_value(true)
                .help("最小星级"),
        )
        .arg(
            Arg::with_name("min-level")
                .long("min-level")
                .takes_value(true)
                .help("最小等级"),
        )
        .arg(
            Arg::with_name("max-wait-switch-relic")
                .long("max-wait-switch-relic")
                .takes_value(true)
                .help("切换遗器最大等待时间(ms)"),
        )
        .arg(
            Arg::with_name("output-dir")
                .long("output-dir")
                .short("o")
                .takes_value(true)
                .help("输出目录")
                .default_value("."),
        )
        .arg(
            Arg::with_name("scroll-stop")
                .long("scroll-stop")
                .takes_value(true)
                .help("翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项，默认为80）"),
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .takes_value(true)
                .help("指定遗器数量（在自动识别数量不准确时使用）"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .help("显示详细信息"),
        )
        .arg(
            Arg::with_name("offset-x")
                .long("offset-x")
                .takes_value(true)
                .help("人为指定横坐标偏移（截图有偏移时可用该选项校正）"),
        )
        .arg(
            Arg::with_name("offset-y")
                .long("offset-y")
                .takes_value(true)
                .help("人为指定纵坐标偏移（截图有偏移时可用该选项校正）"),
        )
        .arg(
            Arg::with_name("output-format")
                .long("output-format")
                .short("f")
                .takes_value(true)
                .help("输出格式")
                .possible_values(&["march7th"])
                .default_value("march7th"),
        )
        .arg(
            Arg::with_name("cloud-wait-switch-relic")
                .long("cloud-wait-switch-relic")
                .takes_value(true)
                .help("指定云·崩坏：星穹铁道切换遗器等待时间(ms)"),
        )
        .get_matches();
    let config = YasScannerConfig::from_match(&matches);

    let rect: Rect;
    let is_cloud: bool;
    let ui: UI;

    #[cfg(windows)]
    {
        use winapi::um::winuser::{SetForegroundWindow, ShowWindow, SW_RESTORE};
        // use winapi::um::shellscalingapi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

        utils::set_dpi_awareness();

        let hwnd;

        (hwnd, is_cloud) = utils::find_window_local("崩坏：星穹铁道")
            .or_else(|_| utils::find_window_local("Honkai: Star Rail"))
            .map(|hwnd| (hwnd, false))
            .unwrap_or_else(|_| {
                let Ok(hwnd) = utils::find_window_cloud() else {
                    utils::error_and_quit("未找到崩坏：星穹铁道窗口，请确认崩坏：星穹铁道已经开启")
                };
                (hwnd, true)
            });

        unsafe {
            ShowWindow(hwnd, SW_RESTORE);
        }
        // utils::sleep(1000);
        unsafe {
            SetForegroundWindow(hwnd);
        }
        utils::sleep(1000);

        rect = utils::get_client_rect(hwnd).unwrap();
        ui = UI::Desktop;
    }

    #[cfg(all(target_os = "linux"))]
    {
        let window_id = unsafe {
            String::from_utf8_unchecked(
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(r#" xwininfo|grep "Window id"|cut -d " " -f 4 "#)
                    .output()
                    .unwrap()
                    .stdout,
            )
        };
        let window_id = window_id.trim_end_matches("\n");

        let position_size = unsafe {
            String::from_utf8_unchecked(
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&format!(r#" xwininfo -id {window_id}|cut -f 2 -d :|tr -cd "0-9\n"|grep -v "^$"|sed -n "1,2p;5,6p" "#))
                    .output()
                    .unwrap()
                    .stdout,
            )
        };

        let mut info = position_size.split("\n");

        let left = info.next().unwrap().parse().unwrap();
        let top = info.next().unwrap().parse().unwrap();
        let width = info.next().unwrap().parse().unwrap();
        let height = info.next().unwrap().parse().unwrap();

        rect = Rect {
            left,
            top,
            width,
            height,
        };
        is_cloud = false; // todo: detect cloud starrail by title
        ui = UI::Desktop;
    }

    #[cfg(target_os = "macos")]
    {
        let (pid, ui_) = get_pid_and_ui();
        let window_title: String;
        (rect, window_title) = unsafe { utils::find_window_by_pid(pid).unwrap() };
        info!("Found starrail pid:{}, window name:{}", pid, window_title);
        is_cloud = false; // todo: detect cloud starrail by title
        ui = ui_;
    }

    // rect.scale(1.25);
    info!(
        "left = {}, top = {}, width = {}, height = {}",
        rect.left, rect.top, rect.width, rect.height
    );

    let mut info: starrail::ScanInfoStarRail;

    // desktop ui or mobile ui
    match ui {
        UI::Desktop => {
            info!("desktop ui");
            info = starrail::ScanInfoStarRail::from_pc(
                rect.width as u32,
                rect.height as u32,
                rect.left,
                rect.top,
            );
        },
        UI::Mobile => {
            info!("mobile ui");
            info = starrail::ScanInfoStarRail::from_mobile(
                rect.width as u32,
                rect.height as u32,
                rect.left,
                rect.top,
            );
        },
    }

    let offset_x = matches
        .value_of("offset-x")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();
    let offset_y = matches
        .value_of("offset-y")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();
    info.left += offset_x;
    info.top += offset_y;

    let model = include_bytes!("../models/model_training.onnx");
    let content = String::from(include_str!("../models/index_2_word.json"));

    let mut scanner = YasScanner::new(info.clone(), config, is_cloud, model, content);

    let now = SystemTime::now();
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到崩坏：星穹铁道窗口，yas将在10s后开始扫描遗器");
        utils::sleep(10000);
    }
    let results = scanner.start();
    let t = now.elapsed().unwrap().as_secs_f64();
    info!("time: {}s", t);

    let output_dir = Path::new(matches.value_of("output-dir").unwrap());
    match matches.value_of("output-format") {
        Some("march7th") => {
            let output_filename = output_dir.join("march7th.json");
            let march7th = March7thFormat::new(&results);
            march7th.save(String::from(output_filename.to_str().unwrap()));
        },
        _ => unreachable!(),
    }
    // let info = info;
    // let img = info.relic_count_position.capture_relative(&info).unwrap();

    // let mut inference = CRNNModel::new(String::from("model_training.onnx"), String::from("index_2_word.json"));
    // let s = inference.inference_string(&img);
    // println!("{}", s);
    info!("识别结束，请按Enter退出");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
}
