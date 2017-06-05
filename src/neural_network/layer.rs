extern crate nalgebra;
extern crate rand;

use self::nalgebra::core::{DMatrix, DVector};
use self::rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct InputLayer {
    
}

#[derive(Debug)]
pub struct HiddenLayer {
    pub weights: DMatrix<f64>,
    pub biases: DVector<f64>,
}

#[derive(Debug)]
pub struct OutputLayer {

}

pub trait CanFeedforward {
    fn feedforward(&self, DVector<f64>) -> DVector<f64>;
}

pub trait CanBackpropagate {
    fn backpropagate(&self, DVector<f64>) -> DVector<f64>;
}

/// T is result type
pub trait CanComputeSolution<T> {
    fn compute(&self, input: DVector<f64>) -> T;
}

impl CanFeedforward for InputLayer {
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

impl CanFeedforward for HiddenLayer {
    fn feedforward(&self, input: DVector<f64>) -> DVector<f64> {

        // println!("input size: \t\t\t{}", input.len());
        // println!("weights size (rows, columns): \t{:?}", self.weights.shape());
        // println!("biases size: \t\t\t{}", self.biases.len());

        let result = &self.weights * input + &self.biases;
        result
    }
}

impl HiddenLayer {
    pub fn new(input_size: usize, neuron_count: usize) -> HiddenLayer {
        let mut rng = thread_rng();
        let generate = |_: usize, _: usize| rng.next_f64() * 2.0 - 1.0;
        let weights = DMatrix::from_fn(neuron_count, input_size, generate);
        let biases = DVector::from_element(neuron_count, 1.0);
        HiddenLayer {
            weights: weights,
            biases: biases,
        }
    }
}

impl CanComputeSolution<usize> for OutputLayer {
    fn compute(&self, input: DVector<f64>) -> usize {
        let mut max_index = 0;
        let mut max_x = input[0];
        for (index, x) in input.iter().enumerate() {
            if *x > max_x {
                max_index = index;
                max_x = *x;
            }
        }
        max_index
    }
}

impl OutputLayer {
    #[allow(unused_variables)]
    pub fn new(input_size: usize) -> OutputLayer {
        let layer = OutputLayer {};
        layer
    }
}