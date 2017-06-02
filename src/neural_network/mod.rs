extern crate nalgebra;
extern crate rand;

use self::nalgebra::core::{DMatrix, DVector};
use self::rand::{thread_rng, Rng};

/// Represents neural network
pub struct NeuralNetwork {
    pub layers: Vec<Box<IsLayer>>
}

#[derive(Debug)]
struct InputLayer {
    
}

#[derive(Debug)]
struct Layer {
    pub weights: DMatrix<f64>,
    pub biases: DVector<f64>
}

pub trait IsLayer {
  fn feedforward(&self, Vec<f64>) -> Vec<f64>;
}

impl IsLayer for InputLayer {
  fn feedforward(&self, input: Vec<f64>) -> Vec<f64> {
    input
  }
}

impl InputLayer {
  fn new(input_size: usize, neuron_count: usize) -> InputLayer {
    let layer = InputLayer { };
    layer
  }
}

impl IsLayer for Layer {
  fn feedforward(&self, input: Vec<f64>) -> Vec<f64> {
    input
  }
}

impl Layer {
  fn new(input_size: usize, neuron_count: usize) -> Layer {
    let mut rng = thread_rng();
    let generate = |x: usize, y: usize| rng.next_f64() * 2.0 - 1.0;
    let weights = DMatrix::from_fn(input_size, neuron_count, generate);
    let biases = DVector::from_element(neuron_count, 1.0);
    Layer { weights:  weights, biases: biases}
  }
}

impl NeuralNetwork {
  pub fn new(input_size: usize, output_size: usize) -> NeuralNetwork {
    let mut neural_network = NeuralNetwork { 
      layers: vec![]
    };

    let input_layer = InputLayer::new(input_size, input_size);
    neural_network.layers.push(Box::new(input_layer));
    let hidden_layer_1 = Layer::new(input_size, output_size);
    neural_network.layers.push(Box::new(hidden_layer_1));
    neural_network
  }
    
  pub fn feedforward(&self, input: Vec<f64>) -> Vec<f64> {
    self.layers.iter().fold(input, |prev_output, layer| layer.feedforward(prev_output))
  }
}