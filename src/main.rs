use std::io::stdin;
use std::path::Path;
use std::time::SystemTime;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;
use yas::capture::capture_absolute_image;
use yas::common::utils;
//use yas::inference::pre_process::{image_to_raw};
use yas::info::info;
//use yas::common::{RawImage};
use yas::scanner::yas_scanner::{YasScanner, YasScannerConfig};

use yas::expo::mingyu_lab::MingyuLabFormat;
use yas::expo::mona_uranai::MonaFormat;

use winapi::um::winuser::{SetForegroundWindow, SetProcessDPIAware, ShowWindow, SW_RESTORE};

use winapi::um::shellscalingapi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

use clap::{App, Arg};

//use image::imageops::grayscale;
use env_logger::Builder;
use log::{info, LevelFilter};
use os_info;

#[derive(Default, NwgUi)]
pub struct YasApp {
    #[nwg_control(size: (300, 180), position: (300, 300), title: "Yas Artifact Scanner", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [YasApp::exit], OnInit: [YasApp::init])]
    window: nwg::Window,

    #[nwg_control(text: "莫娜占卜铺格式(默认)", flags: "VISIBLE|GROUP", size: (150, 35), position: (10, 10), check_state: nwg::RadioButtonState::Checked)]
    mona: nwg::RadioButton,
    #[nwg_control(text: "原魔计算器格式", flags: "VISIBLE", size: (130, 35), position: (160, 10))]
    ymlab: nwg::RadioButton,

    #[nwg_control(text: "最大行数", flags: "VISIBLE", size: (150, 22), position: (10, 50))]
    max_row_label: nwg::Label,
    #[nwg_control(text: "1000", flags: "VISIBLE", size: (130, 22), position: (160, 50))]
    max_row: nwg::TextInput,

    #[nwg_control(text: "最低扫描圣遗物等级", flags: "VISIBLE", size: (150, 22), position: (10, 77))]
    min_level_label: nwg::Label,
    #[nwg_control(text: "5", flags: "VISIBLE", size: (130, 22), position: (160, 77))]
    min_level: nwg::TextInput,

    #[nwg_control(text: "开始扫描", size: (280, 70), position: (10, 104))]
    #[nwg_events( OnButtonClick: [YasApp::scan] )]
    scan_button: nwg::Button,
}

impl YasApp {
    fn init(&self) {
        let mut font = nwg::Font::default();

        nwg::Font::builder()
            .size(18)
            .family("Microsoft YaHei")
            .weight(1000)
            .build(&mut font)
            .unwrap();
        self.mona.set_font(Some(&font));
        self.ymlab.set_font(Some(&font));
        self.max_row.set_font(Some(&font));
        self.max_row_label.set_font(Some(&font));
        self.min_level.set_font(Some(&font));
        self.min_level_label.set_font(Some(&font));
        nwg::Font::builder()
            .size(24)
            .family("Microsoft YaHei")
            .weight(1000)
            .build(&mut font)
            .unwrap();
        self.scan_button.set_font(Some(&font));
    }

    fn scan(&self) {
        let mut config = YasScannerConfig::default();
        config.max_row = match self.max_row.text().parse::<u32>() {
            Ok(n) => {
                if n > 1500 {
                    1500
                } else {
                    n
                }
            }
            _ => 1000,
        };

        config.min_star = match self.min_level.text().parse::<u32>() {
            Ok(n) => {
                if n > 5 {
                    5
                } else {
                    n
                }
            }
            _ => 5,
        };

        if self.ymlab.check_state() == nwg::RadioButtonState::Checked {
            config.format = Some(String::from("mingyulab"));
        }
        do_scan(config);
        nwg::modal_info_message(&self.window, "Done", "识别结束!");
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

// fn open_local(path: String) -> RawImage {
//     let img = image::open(path).unwrap();
//     let img = grayscale(&img);
//     let raw_img = image_to_raw(img);

//     raw_img
// }

fn set_dpi_awareness() {
    let os = os_info::get();

    // unsafe  {
    //     SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    // }
    if os.version() >= &os_info::Version::from_string("8.1") {
        info!("Windows version >= 8.1");
        unsafe {
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

fn do_scan(config: YasScannerConfig) {
    set_dpi_awareness();

    let hwnd = match utils::find_window(String::from("原神")) {
        Err(_s) => {
            utils::error_and_quit("未找到原神窗口，请确认原神已经开启");
        }
        Ok(h) => h,
    };

    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }
    // utils::sleep(1000);
    unsafe {
        SetForegroundWindow(hwnd);
    }
    utils::sleep(1000);

    let rect = utils::get_client_rect(hwnd).unwrap();

    // rect.scale(1.25);
    info!("detected left: {}", rect.left);
    info!("detected top: {}", rect.top);
    info!("detected width: {}", rect.width);
    info!("detected height: {}", rect.height);

    // let _temp = capture_absolute_image(&rect).unwrap().save("test.png");

    let mut info: info::ScanInfo;
    if rect.height * 16 == rect.width * 9 {
        info =
            info::ScanInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else if rect.height * 8 == rect.width * 5 {
        info = info::ScanInfo::from_8_5(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else if rect.height * 4 == rect.width * 3 {
        info = info::ScanInfo::from_4_3(rect.width as u32, rect.height as u32, rect.left, rect.top);
    } else {
        utils::error_and_quit("不支持的分辨率");
    }

    // let offset_x = config.offset_x;
    // let offset_y = config.offset_y;
    info.left += config.offset_x;
    info.top += config.offset_y;

    let output_format = config.format.clone();
    let output_dir = config.output_dir.clone();

    let mut scanner = YasScanner::new(info.clone(), config);

    let now = SystemTime::now();
    let results = scanner.start();
    let t = now.elapsed().unwrap().as_secs_f64();
    info!("time: {}s", t);

    let output_format = output_format.expect("Unreachable");
    let output_dir = output_dir.expect("Unreachable");
    let output_dir = Path::new(&output_dir);

    // if let Some(config_output_dir) = config.output_dir {
    //     output_dir = Path::new(&config_output_dir);
    // }

    match output_format.as_str() {
        "mona" => {
            let output_filename = output_dir.join("mona.json");
            let mona = MonaFormat::new(&results);
            mona.save(String::from(output_filename.to_str().unwrap()));
        }
        "mingyulab" => {
            let output_filename = output_dir.join("mingyulab.json");
            let mingyulab = MingyuLabFormat::new(&results);
            mingyulab.save(String::from(output_filename.to_str().unwrap()));
        }
        _ => (),
    }
    // let info = info;
    // let img = info.art_count_position.capture_relative(&info).unwrap();

    // let mut inference = CRNNModel::new(String::from("model_training.onnx"), String::from("index_2_word.json"));
    // let s = inference.inference_string(&img);
    // println!("{}", s);
}

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();
    let args: Vec<String> = std::env::args().collect();

    let version = get_version();

    let matches = App::new("YAS - 原神圣遗物导出器")
        .version(version.as_str())
        .author("wormtql <584130248@qq.com>")
        .about("Genshin Impact Artifact Exporter")
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
                .help("最小星级")
                .min_values(1)
                .max_values(5),
        )
        .arg(
            Arg::with_name("max-wait-switch-artifact")
                .long("max-wait-switch-artifact")
                .takes_value(true)
                .min_values(10)
                .help("切换圣遗物最大等待时间(ms)"),
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
                .help("指定圣遗物数量（在自动识别数量不准确时使用）")
                .min_values(1)
                .max_values(1500),
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
                .help("输出格式。mona：莫纳占卜铺（默认）；mingyulab：原魔计算器。")
                .possible_values(&["mona", "mingyulab"])
                .default_value("mona"),
        )
        .get_matches();

    if !utils::is_admin() {
        utils::run_as_admin_exit()
    }

    if utils::is_console() == true && args.len() == 1 {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

        let _app = YasApp::build_ui(Default::default()).expect("Failed to build UI");

        nwg::dispatch_thread_events();
    } else {
        let config = YasScannerConfig::from_match(&matches);

        do_scan(config);
        info!("识别结束，请按Enter退出");
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
    }
}
