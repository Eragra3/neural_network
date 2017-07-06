extern crate nalgebra;
extern crate rand;
extern crate pbr;

use super::layer::*;

use self::pbr::ProgressBar;
use std::time::Duration;
use self::nalgebra::core::DVector;
use super::super::mnist_reader::{MnistImage};
// use self::rand::{thread_rng, Rng};

/// Represents neural network
pub struct NeuralNetwork {
    pub input_layer: InputLayer,
    pub hidden_layers: Vec<HiddenLayer<f64>>,
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
        //we treat output size as hidden layer. Output layer only computes solution
        let hidden_layers_sizes = &layers_sizes[1..layers_sizes.len()];

        let input_layer = InputLayer::new(input_size);
        let output_layer = OutputLayer::new(output_size);
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

    pub fn get_correct_solutions(&self, test_set: &Vec<MnistImage>) -> usize {
        let mut pb = ProgressBar::new(test_set.len() as u64);
        pb.set_max_refresh_rate(Some(Duration::from_millis(250)));
        let mut correct_solutions = 0;
        for test_example in test_set {
            let solution = self.feedforward(test_example.data.clone());
            if solution == test_example.label {
                correct_solutions += 1;
            }
            pb.inc();
        }
        pb.finish_println("");
        correct_solutions
    }

    pub fn dump(&self) {
        println!("Neural network");
        println!("\t hidden layers - {}", self.hidden_layers.capacity());
        // println!("\t input layer size - {}", self.input_layer.get_size());
        print!("\t hidden layers sizes (Col x Row) - ");
        for hidden_layer in &self.hidden_layers {
            let (r, c) = hidden_layer.get_size();
            print!("{}x{} ", c, r);
        }
        println!("");
    }
}