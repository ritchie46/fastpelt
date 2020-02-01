
# Pruned Exact Linear Time
*written in rust*

Original [paper](https://arxiv.org/pdf/1101.1438.pdf). 

This is a port of the [ruptures](https://github.com/deepcharles/ruptures) Python project.

## Python

`pip install fastpelt`


```python
import fastpelt
mfast = fastpelt.Pelt(pen=10, loss="l1")
mfast.predict(a)
```

## Rust 

* [API documentation](https://docs.rs/pelt/)

```rust 
extern crate pelt;
use std::fs;
use pelt::{MutEstimator, Pelt};

fn main() {
    let s = fs::read_to_string("./pelt-rs/signal.txt").unwrap();
    let signal: Vec<Vec<f64>> = vec!(s.split("\n").map(|a| a.parse().unwrap()).collect());

    let jump = Some(5);
    let min_size = Some(2);
    let loss = Some("l1");
    let pen = 3.;

    let mut m = Pelt::new(jump, min_size, loss, pen);
    println!("{:?}", m.predict(&signal))
}
```

```text
Some([100, 200])
```

## Performance

![](img/cmp.png)


