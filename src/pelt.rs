use crate::estimator::MutEstimator;

#[derive(Debug)]
struct Pelt {
    jump: u32,
    min_size: u32,
    n_samples: usize,
    best_partition: Option<Vec<usize>>,
}

impl Pelt {
    fn new(jump: Option<u32>, min_size: Option<u32>) -> Pelt {
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
    fn fit(&mut self, signal: Vec<f64>) -> &Self {
        self.n_samples = signal.len();
        self
    }

    fn predict(&mut self, signal: Vec<f64>) -> Vec<f64> {
        vec![0.]
    }

    fn fit_predict(&mut self, signal: Vec<f64>) -> Vec<f64> {
        vec![0.]
    }
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
}
