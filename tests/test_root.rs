#![cfg(test)]
extern crate nrfind;

use nrfind::find_root;

// The function whose root is being determined
fn f(x: f64) -> f64 {
    x.powi(3) + x.powi(2) + 1.0
}

// The derivative of f
fn fd(x: f64) -> f64 {
    (3.0 * x.powi(2)) + (2.0 * x)
}

#[test]
pub fn test_find_root_fails_with_precision() {
    find_root(&f, &fd, 100.0, 0.001, 10).unwrap_err();
}

#[test]
pub fn test_find_root_finds_result_to_precision() {
    let precision = 0.1;
    let result = find_root(&f, &fd, 100.0, precision, 18).unwrap();

    let actual: f64 = -1.4656;
    let difference = (actual - result).abs();
    assert!(difference <= precision);
}

