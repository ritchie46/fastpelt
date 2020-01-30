use crate::estimator::MutEstimator;
use std::collections::HashMap;
use crate::cost;

macro_rules! dict(
{ $($key:expr => $value:expr),+} => {{
    let mut m = HashMap::new();
    $(
       m.insert($key, $value);
    )+
    m
}}
);

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

    fn segmentation(&self, lambda: f64, signal: &Vec<f64>) -> Vec<usize> {
        let idx = proposed_idx(self.n_samples, self.jump, self.min_size);

        // Maps (t, breakpoint) to loss + Lambda
        let first_part = dict!((0, 0) => 0.);
        // partitions[t] contains the optimal partition of signal[0:t]
        let mut partitions_map = dict!(0 => first_part);

        let min_size = self.min_size as f64;
        let jump = self.jump as f64;
        let mut admissible: Vec<usize> = vec![];

        // bp: breakpoint
        for bp in idx {
            // Add points from 0 to current breakpoint as admissible
            // For every slice t:breakpoint we will compute the loss
            // and store it in a hashmap in partitions_map
            let new_adm_pt = (((bp as f64 - min_size) / jump).floor() * jump) as usize;
            admissible.push(new_adm_pt);

            let mut subproblems = vec!();
            for t in &admissible {

                let tmp_part = partitions_map.get(t);
                let mut tmp_part = match tmp_part {
                    // First partition of 0:t doesn't yet exist
                    None => {
                        continue;
                    }
                    Some(v) => v.clone(),
                };

                let loss = cost::l1(signal, *t, bp as usize);
                tmp_part.insert((*t, bp), loss + lambda);

                subproblems.push(tmp_part);
            }

            // Find optimal partition and assign it to partitions_map
            let mut min_part = &subproblems[0];
            let mut min_val = 1e99;
            for (i, d) in subproblems.iter().enumerate() {
                let c = d.values().sum::<f64>();
                if  c < min_val {
                    min_val = c;
                    min_part = &subproblems[i]
                }
            }
            partitions_map.insert(bp, min_part.clone());

            let mut temp_admissible = vec!();
            let sum_part_bp: f64 = partitions_map.get(&bp).unwrap().values().sum();
            for (t, partition) in admissible.iter().zip(subproblems) {
                let c = partition.values().sum::<f64>();
                if c < (sum_part_bp + lambda) {
                    temp_admissible.push(*t);
                }
            }
            admissible.clear();
            admissible.append(&mut temp_admissible);
        }
        let best_part = &partitions_map[&self.n_samples];
        let mut cp: Vec<usize> = best_part.keys().map(|(start, end)| *end).collect();
        cp.sort();
        cp[1..].to_vec()
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
mod _tests {
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

    fn pelt_fixture() -> (Pelt, Vec<f64>) {
        let mut p = Pelt::new(Some(5), Some(2));
        let s = std::fs::read_to_string("signal.txt").unwrap();
        let signal :Vec<f64> = s.split("\n").map(|x|x.parse().unwrap()).collect();
        p.fit(&signal);
        (p, signal)
    }

    #[test]
    fn test_segmentation() {
        let (p, signal) = pelt_fixture();
        let cp = p.segmentation(10., &signal);
        assert_eq!(cp, [100, 200]);
    }
}
