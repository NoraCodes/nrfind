# nrfind
[![Crates.io version badge](https://img.shields.io/crates/v/nrfind.svg)](https://crates.io/crates/nrfind) [![Build Status](https://travis-ci.org/LeoTindall/nrfind.svg?branch=master)](https://travis-ci.org/LeoTindall/nrfind) ![Passively Maintained](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

`nrfind` provides a Newton-Raphson root finder for arbitrary differentiable functions, as well as convenient wrappers for common use cases like square roots.

Documentation is available on [docs.rs](https://docs.rs/nrfind)

A simple example of usage to find the roots of x^3 + x^2 + 1 in 18 iterations:

```rust
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
    let initial_guess = 100.0;
    let precision = 0.1;
    let iterations = 18;

    println!("x^3 + x^2 + 1 = 0 when x ~= {}",
             nrfind::find_root(&f, &fd, 
                               initial_guess, 
                               precision, 
                               iterations).unwrap());
}
```

This will print: `x^3 + x^2 + 1 = 0 when x ~= -1.4675327346575013`.

Note that while this method is guaranteed to approximate _a_ root, it may not be
the root you care about! Changing the given `x0` guess can have an impact on 
which root is approximated.
