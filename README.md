# nrfind
[![Crates.io version badge](https://img.shields.io/crates/v/nrfind.svg)](https://crates.io/crates/nrfind) [![Build Status](https://travis-ci.org/SilverWingedSeraph/nrfind.svg?branch=master)](https://travis-ci.org/SilverWingedSeraph/nrfind)

`nrfind` provides a Newton-Raphson root finder for arbitrary differentiable functions.

Documentation is available on [docs.rs](https://docs.rs/nrfind)

A simple example of usage to find the roots of x^3 + x^2 + 1 in 18 iterations:

```
extern crate nrfind;

// The function for whose roots find_root will solve
fn f(x: f64) -> f64 {
    x.powi(3) + x.powi(2) + 1.0
}

// That function's derivative
fn fd(x: f64) -> f64 {
    (3.0 * x.powi(2)) + (2.0 * x)
}

fn main() {
    let initial_guess = 100;
    let precision = 0.1;
    let iterations = 18;

    let result = nrfind::find_root(&f, &fd, 
                                   &initial_guess, 
                                   &precision, 
                                   iterations).unwrap();

    // This is known to be the actual answer
    let actual: f64 = -1.4656;

    // The difference between the actual answer and the found result
    // is less than the provided precision.
    let difference = (actual - result).abs();
    assert!(difference <= precision);
}
```
