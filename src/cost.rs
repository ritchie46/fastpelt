use crate::consts;

pub fn log_pdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    let d = (x - mean) / std_dev;
    (-0.5 * d * d) - consts::LN_SQRT_2PI - std_dev.ln()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_pdf() {
        assert_eq!(log_pdf(2., 0., 1.), -2.9189385332046727);
        assert_eq!(log_pdf(5., -1., 3.), -4.017550821872782)
    }
}
