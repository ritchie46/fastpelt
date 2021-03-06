extern crate pelt;
use pelt::{MutEstimator, Vec2d, Pelt};
use pyo3::prelude::*;

#[pymodule]
fn fastpeltrust(py: Python, m: &PyModule) -> PyResult<()> {
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values; and the Rust return value back into a Python object.

    #[pyfn(m, "fit_predict")]
    fn fit_predict(
        _py: Python,
        signal: Vec2d,
        min_size: usize,
        loss: &str,
        jump: usize,
        pen: f64,
    ) -> PyResult<Vec<usize>> {
        let mut p = Pelt::new(Some(jump), Some(min_size), Some(loss), pen);
        Ok(p.fit_predict(&signal).unwrap())
    }

    Ok(())
}
