/// Stacked signals
///
/// # Example
/// *One signal:*
/// ```
/// let a: Vec2d = vec!(vec!(1., 2., 3.));
/// ```
///
/// *Two signals:*
/// ```
/// let a: Vec2d = vec!(
/// vec!(1., 2., 3.),
/// vec!(2., 1., 3.)
/// );
/// ```
pub type Vec2d = Vec<Vec<f64>>;

/// Trait mimicking Scikit-Learn API
pub trait MutEstimator<T> {
    fn fit(&mut self, signal: &Vec2d) -> &Self;

    fn predict(&mut self, signal: &Vec2d) -> Option<T>;

    fn fit_predict(&mut self, signal: &Vec2d) -> Option<T>;
}
