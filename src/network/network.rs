use crate::layer::DenseLayer;
use ndarray::*;

pub struct Network {
    pub layers: Vec<DenseLayer>,
}

impl Network {
    pub fn new() -> Self {
        Network { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: DenseLayer) {
        self.layers.push(layer);
    }

    pub fn forward(&self, input: Array1<f32>) -> Array1<f32> {
        let mut output = input;
        for layer in &self.layers {
            output = layer.forward(&output);
        }
        output
    }
}