use tract_onnx::prelude::*;
use anyhow::Result;
use image::{EncodableLayout, RgbImage};
use tract_onnx::tract_hir::infer::InferenceOp;
use tract_onnx::tract_hir::shapefactoid;
use crate::ocr::ImageToText;
use crate::ocr::paddle_paddle_model::preprocess::{normalize_image_to_tensor, resize_img};
use crate::positioning::Shape3D;

// type ModelType = RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;
type ModelType = RunnableModel<InferenceFact, Box<dyn InferenceOp>, Graph<InferenceFact, Box<dyn InferenceOp>>>;

pub struct PPOCRModel {
    index_to_word: Vec<String>,
    model: ModelType,
}

impl PPOCRModel {
    pub fn new(onnx: &[u8], index_to_word: Vec<String>) -> Result<Self> {
        let fact = InferenceFact::new().with_datum_type(DatumType::F32)
            .with_shape(shapefactoid!(_, 3, _, _));
        println!("{}", index_to_word.len());
        let model = tract_onnx::onnx()
            .model_for_read(&mut onnx.as_bytes())?
            .with_input_fact(0, fact)?
            // .into_optimized()?
            .into_runnable()?;
        Ok(Self {
            index_to_word,
            model
        })
    }
}

impl ImageToText<RgbImage> for PPOCRModel {
    fn image_to_text(&self, image: &RgbImage, _is_preprocessed: bool) -> Result<String> {
        let resized_image = resize_img(Shape3D::new(3, 48, 320), &image);
        // resized_image.save("resized.png");
        let tensor = normalize_image_to_tensor(&resized_image);

        let result = self.model.run(tvec!(tensor.into()))?;
        let arr = result[0].to_array_view::<f32>()?;
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
