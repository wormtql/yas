use std::io::stdin;
use std::path::Path;
use std::time::SystemTime;

#[cfg(target_os = "macos")]
use yas_scanner::common::utils::get_pid_and_ui;
use yas_scanner::common::{utils, UI};
use yas_scanner::common::{PixelRect, RawImage};
use yas_scanner::expo::good::GOODFormat;
use yas_scanner::expo::mingyu_lab::MingyuLabFormat;
use yas_scanner::expo::mona_uranai::MonaFormat;

use yas_scanner::inference::pre_process::image_to_raw;
use yas_scanner::info::info;
use yas_scanner::scanner::yas_scanner::{OutputFormat, YasScanner, YasScannerConfig};

use clap::Parser;
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

    let mut config = YasScannerConfig::parse();

    let rect: PixelRect;
    let is_cloud: bool;
    let ui: UI;

    #[cfg(windows)]
    {
        use winapi::um::winuser::{SetForegroundWindow, ShowWindow, SW_RESTORE};
        // use winapi::um::shellscalingapi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

        utils::set_dpi_awareness();

        let hwnd;

        (hwnd, is_cloud) = utils::find_window_local("原神")
            .or_else(|_| utils::find_window_local("Genshin Impact"))
            .map(|hwnd| (hwnd, false))
            .unwrap_or_else(|_| {
                let Ok(hwnd) = utils::find_window_cloud() else {
                    utils::error_and_quit("未找到原神窗口，请确认原神已经开启")
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

        rect = PixelRect {
            left,
            top,
            width,
            height,
        };
        is_cloud = false; // todo: detect cloud genshin by title
        ui = UI::Desktop;
    }

    #[cfg(target_os = "macos")]
    {
        let (pid, ui_) = get_pid_and_ui();
        let window_title: String;
        (rect, window_title) = unsafe { utils::find_window_by_pid(pid).unwrap() };
        info!("Found genshin pid:{}, window name:{}", pid, window_title);
        is_cloud = false; // todo: detect cloud genshin by title
        ui = ui_;
    }

    // rect.scale(1.25);
    info!(
        "left = {}, top = {}, width = {}, height = {}",
        rect.left, rect.top, rect.width, rect.height
    );

    // desktop ui or mobile ui
    let mut info = match ui {
        UI::Desktop => {
            info!("desktop ui");
            info::ScanInfo::from_pc(rect.width as u32, rect.height as u32, rect.left, rect.top)
        },
        UI::Mobile => {
            info!("mobile ui");
            info::ScanInfo::from_mobile(rect.width as u32, rect.height as u32, rect.left, rect.top)
        },
    };

    info.left += config.offset_x;
    info.top += config.offset_y;

    let mut scanner = YasScanner::new(&info, &config, is_cloud);

    let now = SystemTime::now();
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到原神窗口，yas将在10s后开始扫描圣遗物");
        utils::sleep(10000);
    }
    let results = scanner.start();
    let t = now.elapsed().unwrap().as_secs_f64();
    info!("time: {}s", t);

    let output_dir = Path::new(&config.output_dir);
    let output_filename = output_dir.join(config.output_format.filename());
    match config.output_format {
        OutputFormat::Mona => {
            let mona = MonaFormat::new(&results);
            mona.save(String::from(output_filename.to_str().unwrap()));
        },
        OutputFormat::MingyuLab | OutputFormat::Genmo => {
            let mingyulab = MingyuLabFormat::new(&results);
            mingyulab.save(String::from(output_filename.to_str().unwrap()));
        },
        OutputFormat::Good => {
            let good = GOODFormat::new(&results);
            good.save(String::from(output_filename.to_str().unwrap()));
        },
    }
    // let info = info;
    // let img = info.art_count_position.capture_relative(&info).unwrap();

    // let mut inference = CRNNModel::new(String::from("model_training.onnx"), String::from("index_2_word.json"));
    // let s = inference.inference_string(&img);
    // println!("{}", s);
    info!("识别结束，请按Enter退出");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
}
