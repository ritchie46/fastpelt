use crate::consts;

pub fn log_pdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    let d = (x - mean) / std_dev;
    (-0.5 * d * d) - consts::LN_SQRT_2PI - std_dev.ln()
}

pub fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

pub fn var(x: &[f64]) -> f64 {
    let mu = mean(x);
    let mut sum_diff = 0.;

    for v in x {
        sum_diff += (v - mu).powf(2.)
    }
    sum_diff
}

fn nlogn_median(numbers: &[f64]) -> f64 {
    let mut numbers: Vec<f64> = numbers.iter().cloned().collect();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as f64
    } else {
        numbers[mid]
    }
}

fn fast_median(a: &[f64]) -> f64 {
    let mut x = a.to_vec();
    let idx = x.len() / 2;
    *x.partition_at_index_by(idx, |a, b| a.partial_cmp(b).unwrap())
        .1
}

pub fn l2(signal: &[f64], start: usize, end: usize) -> f64 {
    var(&signal[start..end]) * (end - start) as f64
}

pub fn l1(signal: &[f64], start: usize, end: usize) -> f64 {
    let sub = &signal[start..end];
    let med = fast_median(sub);
    sub.iter().map(|a| (a - med).abs()).sum()
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
