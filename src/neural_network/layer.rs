extern crate nalgebra;
extern crate rand;

use self::nalgebra::core::{DMatrix, DVector};
use self::rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct InputLayer {
    
}

#[derive(Debug)]
pub struct Layer {
    pub weights: DMatrix<f64>,
    pub biases: DVector<f64>,
}

pub trait IsLayer {
    fn feedforward(&self, DVector<f64>) -> DVector<f64>;
}

impl IsLayer for InputLayer {
    fn feedforward(&self, input: DVector<f64>) -> DVector<f64> {
        input
    }
}

impl InputLayer {
    #[allow(unused_variables)]
    pub fn new(input_size: usize) -> InputLayer {
        let layer = InputLayer {};
        layer
    }
}

impl IsLayer for Layer {
    fn feedforward(&self, input: DVector<f64>) -> DVector<f64> {

        println!("input size: \t\t\t{}", input.len());
        println!("weights size (rows, columns): \t{:?}", self.weights.shape());
        println!("biases size: \t\t\t{}", self.biases.len());

        let result = &self.weights * input + &self.biases;
        result
    }
}

impl Layer {
    pub fn new(input_size: usize, neuron_count: usize) -> Layer {
        let mut rng = thread_rng();
        let generate = |_: usize, _: usize| rng.next_f64() * 2.0 - 1.0;
        let weights = DMatrix::from_fn(neuron_count, input_size, generate);
        let biases = DVector::from_element(neuron_count, 1.0);
        Layer {
            weights: weights,
            biases: biases,
        }
    }
}