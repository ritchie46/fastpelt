pub type Vec2d = Vec<Vec<f64>>;

pub trait MutEstimator<T> {
    fn fit(&mut self, signal: &Vec2d) -> &Self;

    fn predict(&mut self, signal: &Vec2d) -> Option<T>;

    fn fit_predict(&mut self, signal: &Vec2d) -> Option<T>;
}
