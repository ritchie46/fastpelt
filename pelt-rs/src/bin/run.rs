extern crate pelt;
use std::fs;
use pelt::estimator::MutEstimator;

fn main() {
    let s = fs::read_to_string("./pelt-rs/signal.txt").unwrap();
    let signal: Vec<Vec<f64>> = vec!(s.split("\n").map(|a| a.parse().unwrap()).collect());

    let jump = Some(5);
    let min_size = Some(2);
    let loss = Some("l1");
    let pen = 3.;

    let mut m = pelt::Pelt::new(jump, min_size, loss, pen);
    println!("{:?}", m.predict(&signal))
}
