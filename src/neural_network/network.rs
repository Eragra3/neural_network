extern crate nalgebra;
extern crate rand;

use super::layer::*;

use self::nalgebra::core::DVector;
// use self::rand::{thread_rng, Rng};

/// Represents neural network
pub struct NeuralNetwork {
    pub input_layer: InputLayer,
    pub hidden_layers: Vec<HiddenLayer>,
    pub output_layer: OutputLayer
}

impl NeuralNetwork {
    pub fn new(layers_sizes: Vec<usize>) -> NeuralNetwork {

        match layers_sizes.len() {
            x if x < 2 => panic!("You need to provide at least two layers sizes!"),
            x => println!("Creating neural network with {} layers", x),
        }

        let input_size = layers_sizes[0]; 
        let output_size = layers_sizes[layers_sizes.len() - 1]; 
        let hidden_layers_sizes = &layers_sizes[1..layers_sizes.len() - 1];

        let input_layer = InputLayer::new(input_size);
        let output_layer = OutputLayer::new(input_size);
        let mut hidden_layers = Vec::with_capacity(hidden_layers_sizes.len());

        let mut prev_layer_neuron_count = input_size;
        for layer_size in hidden_layers_sizes {
            let hidden_layer = HiddenLayer::new(prev_layer_neuron_count, *layer_size);
            hidden_layers.push(hidden_layer);
            prev_layer_neuron_count = *layer_size;
        }

        let neural_network = NeuralNetwork { 
            input_layer: input_layer,
            hidden_layers: hidden_layers,
            output_layer: output_layer
        };


        neural_network
    }

    pub fn feedforward(&self, input: DVector<f64>) -> usize {
        let normalized_input = self.input_layer.feedforward(input);

        let iter = self.hidden_layers.iter();
        let output = iter.fold(normalized_input, |prev_output, layer| layer.feedforward(prev_output));

        self.output_layer.compute(output)
    }
}