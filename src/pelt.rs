use crate::estimator::MutEstimator;

#[derive(Debug)]
struct Pelt {
    jump: usize,
    /// Min size of the signal.
    min_size: usize,
    n_samples: usize,
    best_partition: Option<Vec<usize>>,
}

impl Pelt {
    fn new(jump: Option<usize>, min_size: Option<usize>) -> Pelt {
        let jump = match jump {
            Some(v) => v,
            _ => 5,
        };

        let min_size = match min_size {
            Some(v) => v,
            _ => 2,
        };

        Pelt {
            jump,
            min_size,
            n_samples: 0,
            best_partition: None,
        }
    }
}

impl MutEstimator<Vec<f64>> for Pelt {
    fn fit(&mut self, signal: &Vec<f64>) -> &Self {
        self.n_samples = signal.len();
        self
    }

    fn predict(&mut self, signal: &Vec<f64>) -> Vec<f64> {
        vec![0.]
    }

    fn fit_predict(&mut self, signal: &Vec<f64>) -> Vec<f64> {
        vec![0.]
    }
}

/// Proposed changepoint indexes
///
/// # Arguments
/// * `n_samples` - Length of the signal
/// * `jump` - Step size.
/// * `min_size` - Minimal size of the proposed indexes.
fn proposed_idx(n_samples: usize, jump: usize, min_size: usize) -> Vec<usize> {
    let mut idx = vec![];
    for k in (0..n_samples).step_by(jump) {
        if k >= min_size {
            idx.push(k)
        }
    }
    idx.push(n_samples);
    idx
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let p = Pelt::new(None, None);
        assert_eq!(p.min_size, 2);
        assert_eq!(p.jump, 5);

        let p = Pelt::new(Some(50), None);
        assert_eq!(p.jump, 50)
    }

    #[test]
    fn test_proposed_idx() {
        println!("{:?}", proposed_idx(20, 5, 2));
        assert_eq!(proposed_idx(20, 5, 2), vec!(5, 10, 15, 20))
    }
}
