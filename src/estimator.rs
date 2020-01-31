
pub trait MutEstimator<T> {
    fn fit(&mut self, signal: &Vec<f64>) -> &Self;

    fn predict(&mut self, signal: &Vec<f64>) -> Option<T>;

    fn fit_predict(&mut self, signal: &Vec<f64>) -> Option<T>;
}
