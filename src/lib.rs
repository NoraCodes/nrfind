//! `nrfind` is a crate for finding the roots of arbitrary functions whose derivatives
//! are known by the Newton-Rahpson method. 

extern crate num;
use num::Float;
use std::ops::{Div, Sub};

mod sqrt;
pub use sqrt::find_sqrt;

/// Uses the Newton-Raphson method to find roots for `function` given
/// that `derivative` is the first derivative of `function`.
/// If `max_iterations` is reached without the error falling below the given
/// `acceptable_err`, `nrfind` will return `None`.
///
/// This function can operate on any type that implements `Float`. That means
/// arbitrary-precision `BigRational` types from the `num` crate are acceptable.
///
/// # Examples
/// 
/// Here, `find_root` is used to find the root of the polynomial x^3 + x^2 + 1
/// to a precision of 0.1 in 18 iterations, staring from a very
/// inaccurate initial guess.
///
/// ```
/// use nrfind::find_root;
///
/// // The function x^3 + x^2 + 1
/// fn f(x: f64) -> f64 {
///     x.powi(3) + x.powi(2) + 1.0
/// }
///
/// // The derivative of f (3x^2 + 2x)
/// fn fd(x: f64) -> f64 {
///     (3.0 * x.powi(2)) + (2.0 * x)
/// }
///
/// let result = find_root(&f, &fd, 
///                        &100.0,  // Starting guess
///                        &0.1,    // Precision
///                        18       // Iterations
///                       ).unwrap();
/// let actual = -1.4656; // The correct answer
/// let difference = (result - actual).abs();
///
/// assert!(difference < 0.1);
/// ```
/// 
pub fn find_root<N>(function: &Fn(N) -> N, derivative: &Fn(N) -> N, x0: &N, acceptable_err: &N, max_iterations: i32) -> Option<N> 
    where N: Float + Div + Sub + PartialOrd 
{
    let mut current_x: N = *x0;
    let mut next_x: N;

    for _ in 0..max_iterations {
        let deviation = function(current_x) / derivative(current_x);
        next_x = current_x - deviation;
        if deviation.abs() <= *acceptable_err {
            return Some(next_x);
        }
        current_x = next_x;
    }
    return None;
}
