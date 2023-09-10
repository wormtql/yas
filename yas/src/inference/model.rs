use anyhow::Result;
use image::EncodableLayout;
use serde_json::Value;
use tract_onnx::prelude::*;

use crate::core::inference::GrayImageFloat;

type ModelType = RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

/// currently the model is using SVTR structure
pub struct OCRModel {
    model: ModelType,
    index_2_word: Vec<String>,

    pub avg_inference_time: f64,
}

impl OCRModel {
    pub fn new(model: &[u8], content: &str) -> Result<OCRModel> {
        let model = tract_onnx::onnx()
            .model_for_read(&mut model.as_bytes())?
            .with_input_fact(0, f32::fact([1, 1, 32, 384]).into())?
            .into_optimized()?
            .into_runnable()?;

        let json = serde_json::from_str::<Value>(content)?;

        let mut index_2_word = json
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.parse::<usize>().unwrap(), v.as_str().unwrap().to_string()))
            .collect::<Vec<(usize, String)>>();

        index_2_word.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let index_2_word = index_2_word.into_iter().map(|(_, v)| v).collect();

        Ok(OCRModel {
            model,
            index_2_word,
            avg_inference_time: 0.0,
        })
    }

    pub fn inference_string(&self, img: &GrayImageFloat) -> Result<String> {
        let tensor: Tensor =
            tract_ndarray::Array4::from_shape_fn((1, 1, 32, 384), |(_, _, y, x)| {
                img.get_pixel(x as u32, y as u32)[0]
            })
            .into();

        let result = self.model.run(tvec!(tensor))?;
        let arr = result[0].to_array_view::<f32>()?;

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

        Ok(ans)
    }
}
