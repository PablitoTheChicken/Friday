use Friday::prelude::*;
use ndarray::*;

fn main() {
    let mut network = Network::new();
    network.add_layer(DenseLayer::new(2, 3));
    network.add_layer(DenseLayer::new(3, 1));
    
    let input = array![1.0, 2.0];
    let output = network.forward(input);

    println!("{:?}", output);
}
