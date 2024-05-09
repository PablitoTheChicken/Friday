use ndarray::*;
use rand::{self, Rng};

pub struct DenseLayer {
    pub weights: Array2<f32>,
    pub biases: Array1<f32>,
}

impl DenseLayer {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let weights = Array2::from_shape_fn((output_size, input_size), |_| rand::thread_rng().gen_range(-1.0..1.0));
        let biases = Array1::zeros(output_size);
        DenseLayer { weights, biases }
    }

    pub fn forward(&self, input: &Array1<f32>) -> Array1<f32> {
        self.weights.dot(input) + &self.biases
    }
}