#![cfg(test)]
extern crate nrfind;
use nrfind::find_sqrt;

#[test]
pub fn test_find_sqrt_fails_when_out_of_iterations() {
    let radicand = 25.5;
    let precision = 0.001;
    let guess = 100.0; // This guess is far away on purpouse.
    let iterations = 3;
    assert_eq!(None, find_sqrt(&radicand, &guess, &precision, iterations));
}

#[test]
pub fn test_find_sqrt_finds_result_to_precision() {
    let radicand = 25.6;
    let precision = 0.1;
    let guess = 5.0; // This is a fairly good guess.
    let iterations = 20;

    let result = find_sqrt(&radicand, &guess, &precision, iterations).unwrap();
    let actual: f64 = 5.0497;

    let difference = (result - actual).abs();
    println!("find_sqrt gave {} (delta {})", result, difference);
    assert!(difference <= precision);
}

