mod neural_network;

use neural_network::{NeuralNetwork};

fn main() {
    let neural_network = NeuralNetwork::new(20, 10);
    println!("Lel c:");

    let input = (0..20).into_iter().map(|x| x as f64).collect::<Vec<f64>>();
    println!("\n input \n \t{:?}", input);
    println!("\n\n output: \n \t{:?}", neural_network.feedforward(input));
}