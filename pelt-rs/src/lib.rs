//! # Changepoint detection with Pruned Exact Linear Time
//! Changepoint detection using the PELT algorithm. This project is a port of the
//! [ruptures](https://github.com/deepcharles/ruptures) Python project. The PELT algorithm is
//! described in this [paper](https://arxiv.org/pdf/1101.1438.pdf).
//!
//! # Example
//!
//! ```
//! extern crate pelt;
//! use std::fs;
//! use pelt::{MutEstimator, Pelt};
//!
//! fn main() {
//!    let s = fs::read_to_string("./pelt-rs/signal.txt").unwrap();
//!    let signal: Vec<Vec<f64>> = vec!(s.split("\n")
//!    .map(|a| a.parse()
//!    .unwrap())
//!    .collect());
//!
//!    let jump = Some(5);
//!    let min_size = Some(2);
//!    let loss = Some("l1");
//!    let pen = 3.;
//!
//!    let mut m = Pelt::new(jump, min_size, loss, pen);
//!    println!("{:?}", m.predict(&signal))
//!}
//! ```
//!
//! # Nightly
//! A nightly release is required for this crate.
//!
//! # Panics
//! This crate panics when the given signal length is smaller than the `min_size` field given
//! to `alg::Pelt`.
#![feature(test)]
#![feature(slice_partition_at_index)]
mod alg;
mod cost;
mod estimator;
pub use alg::Pelt;
pub use estimator::{MutEstimator, Vec2d};
