pub trait Estimator<T> {
    fn fit(&self, signal: Vec<f64>) -> &Self;

    fn predict(&self, signal: Vec<f64>) -> T;

    fn fit_predict(&self, signal: Vec<f64>) -> T;
}
