extern crate nalgebra;
extern crate rand;

use super::layer::*;

use self::nalgebra::core::DVector;
// use self::rand::{thread_rng, Rng};

/// Represents neural network
pub struct NeuralNetwork {
    pub layers: Vec<Box<IsLayer>>,
}

impl NeuralNetwork {
    pub fn new(layers_sizes: Vec<usize>) -> NeuralNetwork {

        match layers_sizes.len() {
            x if x < 2 => panic!("You need to provide at least 2 layers sizes"),
            x => println!("Creating neural network with {} layers", x),
        }

        let mut neural_network = NeuralNetwork { layers: Vec::with_capacity(layers_sizes.len()) };

        let input_size = layers_sizes[0];
        let input_layer = InputLayer::new(input_size);
        neural_network.layers.push(Box::new(input_layer));

        let mut prev_layer_neuron_count = input_size;
        for layer_size in layers_sizes {
            let hidden_layer_1 = Layer::new(prev_layer_neuron_count, layer_size);
            neural_network.layers.push(Box::new(hidden_layer_1));
            prev_layer_neuron_count = layer_size;
        }

        neural_network
    }

    pub fn feedforward(&self, input: DVector<f64>) -> DVector<f64> {
        self.layers.iter().fold(input, |prev_output, layer| layer.feedforward(prev_output))
    }
}