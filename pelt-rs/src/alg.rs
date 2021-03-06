use crate::cost;
extern crate test;
use crate::estimator::{MutEstimator, Vec2d};
use fnv::FnvHashMap;

macro_rules! dict(
{ $($key:expr => $value:expr),+} => {{
    let mut m = FnvHashMap::default();
    $(
       m.insert($key, $value);
    )+
    m
}}
);

/// Pruned Exact Linear Time changepoint detection model.
/// # Arguments
///
///  * `jump` - Distance between proposal changepoints.
///  * `min_size` - Minimal distance between changepoints.
///  * `n_samples` - No. of samples in signal.
///  * `loss` - Loss function to use.
///  * `pen` - Penalty term.
///
///  # Example
///
///  ```
///  let mut m = Pelt::new(jump, min_size, loss, pen);
///  println!("{:?}", m.predict(&signal))
///  ```
pub struct Pelt {
    jump: usize,
    /// Min size of the partitions.
    min_size: usize,
    n_samples: usize,
    loss: fn(signal: &Vec2d, start: usize, end: usize) -> Option<f64>,
    pen: f64,
}

impl Pelt {
    /// Create new Pelt instance. Note that the variable should be created mutable.
    pub fn new(jump: Option<usize>, min_size: Option<usize>, loss: Option<&str>, pen: f64) -> Pelt {
        let jump = match jump {
            Some(v) => v,
            _ => 5,
        };

        let min_size = match min_size {
            Some(v) => v,
            _ => 2,
        };

        let loss = match loss {
            Some(s) => match s {
                "l1" => cost::l1,
                "l2" => cost::l2,
                _ => panic!("Other loss function not implemented."),
            },
            _ => cost::l1,
        };

        Pelt {
            jump,
            min_size,
            n_samples: 0,
            loss,
            pen,
        }
    }

    fn segmentation(&self, signal: &Vec2d) -> Option<Vec<usize>> {
        let idx = proposed_idx(self.n_samples, self.jump, self.min_size);

        // Maps (t, breakpoint) to loss + pen
        let first_part = dict!((0, 0) => 0.);
        // partitions[t] contains the optimal partition of signal[0:t]
        let mut partitions_map = dict!(0 => first_part);

        let min_size = self.min_size as f64;
        let jump = self.jump as f64;
        let mut admissible: Vec<usize> = vec![];
        let loss_fn = self.loss;

        // bp: breakpoint
        for bp in idx {
            // Add points from 0 to current breakpoint as admissible
            // For every slice t:breakpoint we will compute the loss
            // and store it in a hashmap in partitions_map
            let new_adm_pt = (((bp as f64 - min_size) / jump).floor() * jump) as usize;
            admissible.push(new_adm_pt);

            let mut subproblems = vec![];
            // subproblems will be filled with complete partitioning until bp
            // consider admissible of [0, 5, 10, 15]
            // subproblems could look like:
            //  [
            //      {(0, 15): 23.67, (0, 0): 0.0},
            //      {(0, 5): 14.60, (5, 15): 18.48, (0, 0): 0.0},
            //      {(10, 15): 14.17, (0, 10): 19.22, (0, 0): 0.0}
            // ]
            // Note that every sub-dict is a complete partitioning of the input signal
            for t in &admissible {
                let tmp_part = partitions_map.get(t);
                let mut tmp_part = match tmp_part {
                    // First partition of 0:t doesn't yet exist
                    None => {
                        continue;
                    }
                    Some(v) => v.clone(),
                };

                let loss = loss_fn(signal, *t, bp as usize)?;
                tmp_part.insert((*t, bp), loss + self.pen);

                subproblems.push(tmp_part);
            }

            // Find optimal partition and assign it to partitions_map
            let mut min_part = &subproblems[0];
            let mut min_val = 1e99;
            for (i, d) in subproblems.iter().enumerate() {
                let c = d.values().sum::<f64>();
                if c < min_val {
                    min_val = c;
                    min_part = &subproblems[i]
                }
            }
            partitions_map.insert(bp, min_part.clone());

            let loss_current_part: f64 =
                partitions_map.get(&bp).unwrap().values().sum::<f64>() + self.pen;

            admissible = admissible
                .iter()
                .zip(subproblems)
                // get total loss of partition
                .map(|(t, partition)| (t, partition.values().sum::<f64>()))
                // keep elements that have a lower loss than the current partition
                .filter(|(t, sum_loss)| sum_loss < &(loss_current_part))
                // only keep t
                .map(|(t, sum_loss)| *t)
                .collect();
        }
        let best_part = &partitions_map[&self.n_samples];
        let mut cp: Vec<usize> = best_part.keys().map(|(start, end)| *end).collect();
        cp.sort_unstable();
        cp.remove(0);
        Some(cp)
    }
}

impl MutEstimator<Vec<usize>> for Pelt {
    /// Set the number of samples in the signal.
    /// Explicit call is not needed.
    /// # Panics
    /// if `min_size` < `signal.len()`
    fn fit(&mut self, signal: &Vec2d) -> &Self {
        self.n_samples = signal[0].len();
        if self.n_samples < self.min_size {
            panic!("Cannot have min_size partitions larger than signal length.")
        }
        self
    }

    /// Retreive the changepoints in the signal.
    /// # Arguments
    /// * `signal` - Signals stacked in 2D. All signals should be equal length.
    ///
    /// # Panics
    /// if `min_size` < `signal.len()`
    ///
    fn predict(&mut self, signal: &Vec2d) -> Option<Vec<usize>> {
        self.fit(signal);
        self.segmentation(signal)
    }

    /// See [predict](/pelt/estimator/trait.MutEstimator.html#tymethod.predict)
    fn fit_predict(&mut self, signal: &Vec2d) -> Option<Vec<usize>> {
        self.predict(signal)
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
mod _tests {
    use super::*;

    #[test]
    fn test_init() {
        let p = Pelt::new(None, None, None, 10.);
        assert_eq!(p.min_size, 2);
        assert_eq!(p.jump, 5);

        let p = Pelt::new(Some(50), None, None, 10.);
        assert_eq!(p.jump, 50)
    }

    #[test]
    fn test_proposed_idx() {
        println!("{:?}", proposed_idx(20, 5, 2));
        assert_eq!(proposed_idx(20, 5, 2), vec!(5, 10, 15, 20))
    }

    fn pelt_fixture() -> (Pelt, Vec2d) {
        let mut p = Pelt::new(Some(5), Some(2), None, 10.);
        let s = std::fs::read_to_string("signal.txt").unwrap();
        let signal: Vec<Vec<f64>> = vec![s.split("\n").map(|x| x.parse().unwrap()).collect()];
        p.fit(&signal);
        (p, signal)
    }

    #[test]
    fn test_segmentation() {
        let (p, signal) = pelt_fixture();
        let cp = p.segmentation(&signal).unwrap();
        assert_eq!(cp, [100, 200]);
    }
    use test::Bencher;
    #[bench]
    fn bench_segmentation(b: &mut Bencher) {
        let (p, signal) = pelt_fixture();
        b.iter(|| p.segmentation(&signal));
    }
}
