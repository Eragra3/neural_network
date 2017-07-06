mod neural_network;
mod mnist_reader;

extern crate nalgebra;
extern crate rand;

use neural_network::NeuralNetwork;

fn main() {

    println!("Reading mnist training dataset");
    let training_dataset;
    
    match mnist_reader::read("./src/datasets/mnist/train-images.idx3-ubyte", "./src/datasets/mnist/train-labels.idx1-ubyte") {
        Ok(result) => training_dataset = result,
        Err(message) => panic!(format!("MNIST parser failed with '{}'", message))
    }

    println!("Generating neural network");
    let neural_network = NeuralNetwork::new(vec![784, 50, 10]);
    neural_network.dump();

    println!("Testing against MNIST test set");
    let correct_solutions = neural_network.get_correct_solutions(&training_dataset);
    println!("Correct colutions : {}/{}", correct_solutions, training_dataset.capacity());
}