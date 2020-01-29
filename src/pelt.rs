use crate::estimator::Estimator;

#[derive(Debug)]
struct Pelt {
    jump: u64,
    min_size: u64,
}

impl Pelt {
    fn new(jump: Option<u64>, min_size: Option<u64>) -> Pelt {
        let jump = match jump {
            Some(v) => v,
            _ => 5
        };

        let min_size = match min_size {
            Some(v) => v,
            _ => 2
        };

        Pelt{jump, min_size}
    }
}

impl Estimator<Vec<f64>> for Pelt {
    fn fit(&self, signal: Vec<f64>) -> &Self {
        self
    }

    fn predict(&self, signal: Vec<f64>) -> Vec<f64> {
        vec!(0.)
    }

    fn fit_predict(&self, signal: Vec<f64>) -> Vec<f64> {
        vec!(0.)
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
