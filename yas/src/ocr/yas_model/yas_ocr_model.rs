use std::{cell::RefCell, time::Duration};
use std::time::SystemTime;
use image::{EncodableLayout, GrayImage, ImageBuffer, Luma, RgbImage};
// use tract_onnx::prelude::*;
use crate::ocr::traits::ImageToText;
use super::preprocess;
use anyhow::Result;
use crate::common::image_ext::*;
#[cfg(feature = "tract_onnx")]
use tract_onnx::prelude::*;

#[cfg(feature = "tract_onnx")]
type ModelType = RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

pub struct YasOCRModel {
    #[cfg(feature = "ort")]
    model: ort::Session,
    #[cfg(feature = "tract_onnx")]
    model: ModelType,
    index_to_word: Vec<String>,

    inference_time: RefCell<Duration>,   // in seconds
    invoke_count: RefCell<usize>,
}

impl YasOCRModel {
    pub fn get_average_inference_time(&self) -> Option<Duration> {
        let count = *self.invoke_count.borrow();
        let total_time = *self.inference_time.borrow();
        
        if count == 0 {
            None
        } else {
            Some(total_time.div_f64(count as f64))
        }
    }

    pub fn new(model: &[u8], content: &str) -> Result<YasOCRModel> {
        #[cfg(feature = "ort")]
        let model = ort::Session::builder()?
            .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_memory(model)?;
        #[cfg(feature = "tract_onnx")]
        let model = tract_onnx::onnx()
            .model_for_read(&mut model.as_bytes())?
            .with_input_fact(0, f32::fact([1, 1, 32, 384]).into())?
            .into_optimized()?
            .into_runnable()?;

        let json = serde_json::from_str::<serde_json::Value>(content)?;

        let mut index_to_word = json
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.parse::<usize>().unwrap(), v.as_str().unwrap().to_string()))
            .collect::<Vec<(usize, String)>>();

        index_to_word.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let index_to_word = index_to_word.into_iter().map(|(_, v)| v).collect();

        Ok(YasOCRModel {
            model,
            index_to_word,
            inference_time: RefCell::new(Duration::new(0, 0)),
            invoke_count: RefCell::new(0),
        })
    }

    pub fn inference_string(&self, img: &ImageBuffer<Luma<f32>, Vec<f32>>) -> Result<String> {
        let now = SystemTime::now();

        #[cfg(feature = "ort")]
        let tensor = ndarray::Array4::from_shape_fn((1, 1, 32, 384), |(_, _, y, x)| {
            img.get_pixel(x as u32, y as u32)[0]
        });
        #[cfg(feature = "tract_onnx")]
        let tensor: Tensor =
            tract_ndarray::Array4::from_shape_fn((1, 1, 32, 384), |(_, _, y, x)| {
                img.get_pixel(x as u32, y as u32)[0]
            }).into();

        #[cfg(feature = "ort")]
        let result = self.model.run(ort::inputs![tensor]?)?;
        #[cfg(feature = "tract_onnx")]
        let result = self.model.run(tvec!(tensor.into()))?;

        #[cfg(feature = "ort")]
        let arr = result[0].try_extract_tensor()?;
        #[cfg(feature = "tract_onnx")]
        let arr = result[0].to_array_view::<f32>()?;

        let shape = arr.shape();

        let mut ans = String::new();
        let mut last_word = String::new();
        for i in 0..shape[0] {
            let mut max_index = 0;
            let mut max_value = -1.0_f32;
            for j in 0..self.index_to_word.len() {
                let value = arr[[i, 0, j]];
                if value > max_value {
                    max_value = value;
                    max_index = j;
                }
            }
            let word = &self.index_to_word[max_index];
            if *word != last_word && word != "-" {
                ans = ans + word;
            }

            last_word = word.clone();
        }

        let time = now.elapsed()?;
        
        *self.invoke_count.borrow_mut() += 1;
        *self.inference_time.borrow_mut() += time;

        Ok(ans)
    }
}

impl ImageToText<RgbImage> for YasOCRModel {
    fn image_to_text(&self, image: &RgbImage, is_preprocessed: bool) -> Result<String> {
        assert_eq!(is_preprocessed, false);

        let gray_image_float = preprocess::to_gray(image);
        let (result, non_mono) = preprocess::pre_process(gray_image_float);

        if !non_mono {
            return Ok(String::new());
        }

        let string_result = self.inference_string(&result)?;

        Ok(string_result)
    }

    fn get_average_inference_time(&self) -> Option<Duration> {
        self.get_average_inference_time()
    }
}

impl ImageToText<ImageBuffer<Luma<f32>, Vec<f32>>> for YasOCRModel {
    fn image_to_text(&self, image: &ImageBuffer<Luma<f32>, Vec<f32>>, is_preprocessed: bool) -> Result<String> {
        if is_preprocessed {
            let string_result = self.inference_string(image)?;
            Ok(string_result)
        } else {
            let im = image.clone();
            let (preprocess_result, non_mono) = preprocess::pre_process(im);

            if !non_mono {
                return Ok(String::new());
            }

            let string_result = self.inference_string(&preprocess_result)?;
            Ok(string_result)
        }
    }

    fn get_average_inference_time(&self) -> Option<Duration> {
        self.get_average_inference_time()
    }
}

impl ImageToText<GrayImage> for YasOCRModel {
    fn image_to_text(&self, im: &GrayImage, is_preprocessed: bool) -> Result<String> {
        let gray_f32_image: ImageBuffer<Luma<f32>, Vec<f32>> = im.to_f32_gray_image();
        self.image_to_text(&gray_f32_image, is_preprocessed)
    }

    fn get_average_inference_time(&self) -> Option<Duration> {
        self.get_average_inference_time()
    }
}

pub macro yas_ocr_model($model_name:literal, $index_to_word:literal) {
    {
        let model_bytes = include_bytes!($model_name);
        let index_to_word = include_str!($index_to_word);

        YasOCRModel::new(
            model_bytes, index_to_word,
        )
    }
}
