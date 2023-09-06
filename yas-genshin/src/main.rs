use std::io::stdin;
use std::path::Path;
use std::time::SystemTime;

#[cfg(target_os = "macos")]
use yas::common::utils::get_pid_and_ui;
use yas::common::{utils, UI};
use yas::common::{RawImage, Rect};
use yas::export::good::GOODFormat;
use yas::export::mingyu_lab::MingyuLabFormat;
use yas::export::mona_uranai::MonaFormat;

use yas::core::info;
use yas::inference::pre_process::image_to_raw;
use yas::scanner::genshin::{YasScanner, YasScannerConfig};

use clap::{App, Arg};
use image::imageops::grayscale;

use env_logger::Builder;
use log::{info, warn, LevelFilter};

use anyhow::Result;

fn open_local(path: String) -> RawImage {
    let img = image::open(path).unwrap();
    let img = grayscale(&img);
    let raw_img = image_to_raw(img);

    raw_img
}

fn main() -> Result<()> {
    yas::init_env();

    let matches = yas::common_app()
        .arg(
            Arg::with_name("max-wait-switch-artifact")
                .long("max-wait-switch-artifact")
                .takes_value(true)
                .help("切换圣遗物最大等待时间(ms)"),
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .takes_value(true)
                .help("指定圣遗物数量（在自动识别数量不准确时使用）"),
        )
        .arg(
            Arg::with_name("output-format")
                .long("output-format")
                .short("f")
                .takes_value(true)
                .help("输出格式")
                .possible_values(&["mona", "mingyulab", "good"])
                .default_value("mona"),
        )
        .arg(
            Arg::with_name("cloud-wait-switch-artifact")
                .long("cloud-wait-switch-artifact")
                .takes_value(true)
                .help("指定云·原神切换圣遗物等待时间(ms)"),
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

        rect = Rect {
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

    let mut info: info::ScanInfo;

    // desktop ui or mobile ui
    match ui {
        UI::Desktop => {
            info!("desktop ui");
            info =
                info::ScanInfo::from_pc(rect.width as u32, rect.height as u32, rect.left, rect.top);
        },
        UI::Mobile => {
            info!("mobile ui");
            info = info::ScanInfo::from_mobile(
                rect.width as u32,
                rect.height as u32,
                rect.left,
                rect.top,
            );
        },
    }

    let offset_x = matches.value_of("offset-x").unwrap_or("0").parse::<i32>()?;

    let offset_y = matches.value_of("offset-y").unwrap_or("0").parse::<i32>()?;

    info.left += offset_x;
    info.top += offset_y;

    let model = include_bytes!("../models/model_training.onnx");
    let content = String::from(include_str!("../models/index_2_word.json"));

    let mut scanner = YasScanner::new(info.clone(), config, is_cloud, model, content);

    let now = SystemTime::now();
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到原神窗口，yas将在10s后开始扫描圣遗物");
        utils::sleep(10000);
    }
    let results = scanner.start();
    let t = now.elapsed()?.as_secs_f64();
    info!("time: {}s", t);

    let output_dir = Path::new(matches.value_of("output-dir").unwrap());
    match matches.value_of("output-format") {
        Some("mona") => {
            let output_filename = output_dir.join("mona.json");
            let mona = MonaFormat::new(&results);
            mona.save(String::from(output_filename.to_str().unwrap()));
        },
        Some("mingyulab") => {
            let output_filename = output_dir.join("mingyulab.json");
            let mingyulab = MingyuLabFormat::new(&results);
            mingyulab.save(String::from(output_filename.to_str().unwrap()));
        },
        Some("good") => {
            let output_filename = output_dir.join("good.json");
            let good = GOODFormat::new(&results);
            good.save(String::from(output_filename.to_str().unwrap()));
        },
        _ => unreachable!(),
    }

    info!("识别结束，请按Enter退出");
    let mut s = String::new();
    stdin().read_line(&mut s)?;

    Ok(())
}
