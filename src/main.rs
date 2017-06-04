mod neural_network;

extern crate nalgebra;
extern crate rand;

use self::nalgebra::core::{DVector};
use self::rand::{thread_rng, Rng};
use neural_network::NeuralNetwork;

fn main() {

    let mut rng = thread_rng();
    let generate = |_: usize, _: usize| rng.next_f64() * 2.0 - 1.0;

    let input = DVector::from_fn(20, generate);

    let neural_network = NeuralNetwork::new(vec![input.len(), 50, 10]);
    println!("Lel c:");


    println!("\n input \n \t{:?}", input);
    println!("\n\n output: \n \t{:?}", neural_network.feedforward(input));
}