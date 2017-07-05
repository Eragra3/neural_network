mod neural_network;
mod mnist_reader;

extern crate nalgebra;
extern crate rand;

use self::nalgebra::core::{DVector};
use self::rand::{thread_rng, Rng};
use neural_network::NeuralNetwork;

fn main() {

    println!("Reading mnist training dataset");
    let training_dataset;
    
    match mnist_reader::read("./src/datasets/mnist/train-images.idx3-ubyte", "./src/datasets/mnist/train-labels.idx1-ubyte") {
        Ok(result) => training_dataset = result,
        Err(message) => panic!(format!("MNIST parser failed with '{}'", message))
    }

    println!("Generating neural network");
    let mut rng = thread_rng();
    let generate = |_: usize, _: usize| rng.next_f64() * 2.0 - 1.0;

    let input = DVector::from_fn(20, generate);
    
    let neural_network = NeuralNetwork::new(vec![input.len(), 50, 10]);
    println!("Lel c:");


    println!("\n input \n \t{:?}", input);
    println!("\n\n output: \n \t{:?}", neural_network.feedforward(input));
}