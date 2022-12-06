use std::collections::HashMap;
use std::io::Read;

use tract_onnx::prelude::*;
use tract_onnx::Onnx;
use serde_json::{Result, Value};

use crate::common::RawImage;
use crate::common::utils;
use image::EncodableLayout;


type ModelType = RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

pub struct CRNNModel {
    model: ModelType,
    index_2_word: Vec<String>,

    pub avg_inference_time: f64,
}

impl CRNNModel {
    pub fn new(name: String, dict_name: String) -> CRNNModel {
        // let model = tract_onnx::onnx()
        //     .model_for_path(String::from("models/") + name.as_str()).unwrap()
        //     .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 1, 32, 384))).unwrap()
        //     .into_optimized().unwrap()
        //     .into_runnable().unwrap();
        let mut bytes = include_bytes!("../../models/model_acc100-epoch45.onnx");
        // let mut bytes = include_bytes!("../../models/model_training.onnx");

        let model = tract_onnx::onnx()
            .model_for_read(&mut bytes.as_bytes()).unwrap()
            .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 1, 32, 384))).unwrap()
            .into_optimized().unwrap()
            .into_runnable().unwrap();

        // let content = utils::read_file_to_string(String::from("models/index_2_word.json"));
        let content = String::from(include_str!("../../models/index_2_word.json"));
        let json: Value = serde_json::from_str(content.as_str()).unwrap();

        let mut index_2_word: Vec<String> = Vec::new();
        let mut i = 0;
        loop {
            let word = match json.get(i.to_string()) {
                Some(x) => x,
                None => break,
            };
            index_2_word.push(word.as_str().unwrap().to_string());
            i += 1;
        }

        CRNNModel {
            model,
            index_2_word,

            avg_inference_time: 0.0,
        }
    }

    pub fn inference_string(&self, img: &RawImage) -> String {
        let tensor: Tensor = tract_ndarray::Array4::from_shape_fn((1, 1, 32, 384), |(_, _, y, x)| {
            let index = img.w * y as u32 + x as u32;
            img.data[index as usize]
        }).into();

        let result = self.model.run(tvec!(tensor)).unwrap();
        let arr = result[0].to_array_view::<f32>().unwrap();

        let shape = arr.shape();

        let mut ans = String::new();
        let mut last_word = String::new();
        for i in 0..shape[0] {
            let mut max_index = 0;
            let mut max_value = -1.0;
            for j in 0..self.index_2_word.len() {
                let value = arr[[i, 0, j]];
                if value > max_value {
                    max_value = value;
                    max_index = j;
                }
            }
            let word = &self.index_2_word[max_index];
            if *word != last_word && word != "-" {
                ans = ans + word;
            }

            last_word = word.clone();
        }

        ans
    }
}