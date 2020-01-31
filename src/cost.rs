use crate::consts;

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
    Some(sum_diff)
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

pub fn l2(signal: &[f64], start: usize, end: usize) -> Option<f64> {
    if signal.len() == 0 {
        return None;
    }
    let variance = var(&signal[start..end])?;
    Some(variance * (end - start) as f64)
}

pub fn l1(signal: &[f64], start: usize, end: usize) -> Option<f64> {
    if signal.len() == 0 {
        return None;
    }
    let sub = &signal[start..end];
    let med = fast_median(sub)?;
    Some(sub.iter().map(|a| (a - med).abs()).sum())
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
