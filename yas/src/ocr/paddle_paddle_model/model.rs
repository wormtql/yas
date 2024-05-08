use std::cell::RefCell;
use std::path::Path;
use std::time::{Duration, SystemTime};
use ort::GraphOptimizationLevel;
use anyhow::Result;
use image::{EncodableLayout, RgbImage};
use crate::ocr::ImageToText;
use crate::ocr::paddle_paddle_model::preprocess::{normalize_image_to_ndarray, resize_img};
use crate::positioning::Shape3D;
use crate::utils::read_file_to_string;

pub struct PPOCRModel {
    index_to_word: Vec<String>,
    // model: ModelType,
    model: ort::Session,

    inference_count: RefCell<usize>,
    inference_time: RefCell<Duration>,
}

fn parse_index_to_word(s: &str, use_whitespace: bool) -> Vec<String> {
    let mut result = Vec::new();
    for line in s.lines() {
        result.push(String::from(line));
    }
    if use_whitespace {
        result.push(String::from(" "));
    }
    result
}

impl PPOCRModel {
    pub fn new_from_file<P1, P2>(onnx_file: P1, words_file: P2) -> Result<PPOCRModel> where P1: AsRef<Path>, P2: AsRef<Path> {
        let words_str = std::fs::read_to_string(words_file)?;
        let index_to_word = parse_index_to_word(&words_str, true);

        let model = ort::Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(onnx_file)?;

        Ok(Self {
            index_to_word,
            model,
            inference_count: RefCell::new(0),
            inference_time: RefCell::new(Duration::new(0, 0)),
        })
    }

    pub fn new(onnx: &[u8], index_to_word: Vec<String>) -> Result<Self> {
        let model = ort::Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_memory(onnx)?;

        Ok(Self {
            index_to_word,
            model,
            inference_count: RefCell::new(0),
            inference_time: RefCell::new(Duration::new(0, 0)),
        })
    }

    pub fn get_average_inference_time(&self) -> Option<Duration> {
        if *self.inference_count.borrow() == 0 {
            None
        } else {
            let count = *self.inference_count.borrow();
            let duration = self.inference_time.borrow().clone();
            Some(duration.div_f64(count as f64))
        }
    }
}

impl ImageToText<RgbImage> for PPOCRModel {
    fn image_to_text(&self, image: &RgbImage, _is_preprocessed: bool) -> Result<String> {
        let start_time = SystemTime::now();

        let resized_image = resize_img(Shape3D::new(3, 48, 320), &image);
        // resized_image.save("resized.png");
        let tensor = normalize_image_to_ndarray(&resized_image);

        let result = self.model.run(ort::inputs![tensor]?)?;
        let arr = result[0].try_extract_tensor()?;
        let shape = arr.shape();
        // println!("{:?}", shape);

        let mut text_index = Vec::new();

        for i in 0..shape[1] {
            let mut max_index = 0;
            let mut max_value = -f32::INFINITY;
            for j in 0..shape[2] {
                let value = arr[[0, i, j]];
                // println!("{}", value);
                if value > max_value {
                    max_value = value;
                    max_index = j;
                }
            }
            text_index.push(max_index);
        }

        let mut indices = Vec::new();
        if text_index[0] != 0 {
            indices.push(text_index[0]);
        }
        for i in 1..text_index.len() {
            if text_index[i] != text_index[i - 1] && text_index[i] != 0 {
                indices.push(text_index[i]);
            }
        }

        let mut s = String::new();
        for &index in indices.iter() {
            s.push_str(&self.index_to_word[index - 1]);
        }

        // println!("{:?}", text_index);

        // let s = format!("{:?}", shape);

        let elapsed_time = start_time.elapsed()?;
        *self.inference_time.borrow_mut() += elapsed_time;
        *self.inference_count.borrow_mut() += 1;

        Ok(s)
    }
}

pub macro ppocr_model($onnx:literal, $index_to_word:literal) {
    {
        let model_bytes = include_bytes!($onnx);
        let index_to_word_str = include_str!($index_to_word);

        let mut index_to_word_vec: Vec<String> = Vec::new();
        for line in index_to_word_str.lines() {
            index_to_word_vec.push(String::from(line));
        }
        index_to_word_vec.push(String::from(" "));

        PPOCRModel::new(
            model_bytes, index_to_word_vec,
        )
    }
}

pub struct PPOCRChV4RecInfer {
    model: PPOCRModel,
}

impl PPOCRChV4RecInfer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            model: ppocr_model!("./ch_PP-OCRv4_rec_infer.onnx", "./ppocr_keys_v1.txt")?
        })
    }
}

impl ImageToText<RgbImage> for PPOCRChV4RecInfer {
    fn image_to_text(&self, image: &RgbImage, is_preprocessed: bool) -> Result<String> {
        self.model.image_to_text(image, is_preprocessed)
    }
}
