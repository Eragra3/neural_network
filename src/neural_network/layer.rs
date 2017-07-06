extern crate nalgebra;
extern crate rand;

use self::nalgebra::core::{DMatrix, DVector};
use self::rand::{thread_rng, Rng};
use super::math_utils::*;

use std::marker::Copy;
use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait MatrixElement : 'static + Copy + PartialEq + Debug {}
impl<T> MatrixElement for T where T : 'static + Copy + PartialEq + Debug {}

#[derive(Debug)]
pub struct InputLayer {
    
}

#[derive(Debug)]
pub struct HiddenLayer<T: MatrixElement> {
    pub weights: DMatrix<T>,
    pub biases: DVector<T>,
}

#[derive(Debug)]
pub struct OutputLayer {

}

pub trait CanFeedforward<T: MatrixElement> {
    fn feedforward(&self, DVector<T>) -> DVector<T>;
}

pub trait CanActivate<T: MatrixElement> {
    fn activate(&self, DVector<T>) -> DVector<T>;
}

pub trait CanBackpropagate<T: MatrixElement> {
    fn backpropagate(&self, DVector<T>) -> DVector<T>;
}

/// T is result type
pub trait CanComputeSolution<T: MatrixElement, S> {
    fn compute(&self, input: DVector<T>) -> S;
}

impl CanFeedforward<f64> for InputLayer {
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

    // pub fn get_size(&self) -> usize {
    //     0
    // }
}

impl CanFeedforward<f64> for HiddenLayer<f64> {
    fn feedforward(&self, input: DVector<f64>) -> DVector<f64> {
        let result = &self.weights * input + &self.biases;
        self.activate(result)
    }
}

impl CanActivate<f64> for HiddenLayer<f64> {
    fn activate(&self, input: DVector<f64>) -> DVector<f64> {
        input.map(|v| sigmoid(v))
    }
}

impl HiddenLayer<f64> {
    pub fn new(input_size: usize, neuron_count: usize) -> HiddenLayer<f64> {
        let mut rng = thread_rng();
        let generate = |_: usize, _: usize| rng.next_f64() * 2.0 - 1.0;
        let weights = DMatrix::from_fn(neuron_count, input_size, generate);
        let biases = DVector::from_element(neuron_count, 1.0);
        HiddenLayer {
            weights: weights,
            biases: biases,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.weights.nrows(), self.weights.ncols())
    }
}

impl CanComputeSolution<f64, usize> for OutputLayer {
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