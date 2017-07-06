pub fn sigmoid(value: f64) -> f64 {
    1. / (1. + (-value).exp())
}

pub fn sigmoid_derivative(value: f64) -> f64 {
    sigmoid(value) * (1. - sigmoid(value))
}

pub fn tanh(value: f64) -> f64 {
    value.tanh()
}

pub fn relu(value: f64) -> f64 {
    if value < 0.  { 
        0.
    } else {
        value
    }
}