use crate::consts;
use crate::estimator::Vec2d;

pub fn log_pdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    let d = (x - mean) / std_dev;
    (-0.5 * d * d) - consts::LN_SQRT_2PI - std_dev.ln()
}

fn mean(x: &[f64]) -> Option<f64> {
    match x {
        [] => None,
        _ => Some(x.iter().sum::<f64>() / x.len() as f64),
    }
}

fn var(x: &[f64]) -> Option<f64> {
    let mu = mean(x)?;
    let mut sum_diff = 0.;

    for v in x {
        sum_diff += (v - mu).powf(2.)
    }
    Some(sum_diff / x.len() as f64)
}

fn nlogn_median(numbers: &[f64]) -> Option<f64> {
    let mut numbers: Vec<f64> = numbers.iter().cloned().collect();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]])
    } else {
        Some(numbers[mid])
    }
}

fn fast_median(a: &[f64]) -> Option<f64> {
    let mut x = a.to_vec();
    let idx = x.len() / 2;
    Some(
        *x.partition_at_index_by(idx, |a, b| a.partial_cmp(b).unwrap())
            .1,
    )
}

pub fn l2(signal: &Vec2d, start: usize, end: usize) -> Option<f64> {
    let mut variance = 0.;
    for s in signal {
        if s.len() == 0 {
            return None;
        }
        variance += var(&s[start..end])?;
    }

    Some(variance * (end - start) as f64)
}

pub fn l1(signal: &Vec2d, start: usize, end: usize) -> Option<f64> {
    let mut loss = 0.;
    for s in signal {
        if s.len() == 0 {
            return None;
        }
        let sub = &s[start..end];
        let med = fast_median(sub)?;
        loss += sub.iter().map(|a| (a - med).abs()).sum::<f64>()
    }
    Some(loss)
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
