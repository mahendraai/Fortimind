// This file contains logic for AI model inference, utilizing ONNX models to analyze data and make predictions.

use onnxruntime::{environment::Environment, session::Session, tensor::Tensor};
use std::error::Error;

pub struct InferenceModel {
    session: Session,
}

impl InferenceModel {
    pub fn new(model_path: &str) -> Result<Self, Box<dyn Error>> {
        let environment = Environment::builder()
            .with_name("InferenceModel")
            .build()?;
        let session = environment.new_session(model_path)?;
        Ok(InferenceModel { session })
    }

    pub fn predict(&self, input_data: Vec<f32>) -> Result<Vec<f32>, Box<dyn Error>> {
        let input_tensor = Tensor::from_slice(&input_data, &[1, input_data.len()])?;
        let outputs = self.session.run(vec![input_tensor])?;
        let output_tensor: &Tensor<f32> = outputs[0].try_extract()?;
        Ok(output_tensor.to_vec())
    }
}