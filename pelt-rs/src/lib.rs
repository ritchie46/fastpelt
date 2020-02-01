#![feature(test)]
#![feature(slice_partition_at_index)]
mod consts;
pub mod cost;
pub mod estimator;
pub mod alg;
pub use alg::Pelt as Pelt;
