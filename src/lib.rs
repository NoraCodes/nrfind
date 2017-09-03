//! `nrfind` is a crate for finding the roots of arbitrary functions whose derivatives
//! are known by the Newton-Rahpson method. 
//!
//! # Examples
//!
//! Here, `find_sqrt` is used to approximate the square root of 25.1 to within
//! 0.01, with a starting guess of 5, using at most 100 iterations of the NR
//! method.
//!
//! ```
//! use nrfind::find_sqrt;
//!
//! match find_sqrt(25.1, 5.0, 0.01, 100) {
//!     Ok(v) => println!("sqrt(25.1) ~= {} within 0.01", v),
//!     Err(v) => println!("Could not find sqrt(25.1) within 0.01; closest guess was {}.", v),
//! }
//! ```
//!

extern crate num;
use num::Float;
use std::ops::{Div, Sub};

mod sqrt;
pub use sqrt::find_sqrt;

/// A `NumericSolutionResult` is returned from the various `nrfind` functions.
/// An `Ok` value signifies a result that is within the given precision while an
/// `Err` result signifies a result that is not known to be sufficiently precise.
pub type NumericSolutionResult<T: Float + Div + Sub + PartialOrd> = Result<T, T>;

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
///                        100.0,   // Starting guess
///                        0.1,     // Precision
///                        18       // Iterations
///                       ).unwrap();
/// let actual = -1.4656; // The correct answer
///
/// let difference = (result - actual).abs();
///
/// assert!(difference < 0.1);
/// ```
/// 
pub fn find_root<N>(function: &Fn(N) -> N, derivative: &Fn(N) -> N, x0: N, acceptable_err: N, max_iterations: i32) -> NumericSolutionResult<N>
    where N: Float + Div + Sub + PartialOrd 
{
    let mut current_x: N = x0;
    let mut next_x: N;

    for _ in 0..max_iterations {
        let deviation = function(current_x) / derivative(current_x);
        next_x = current_x - deviation;
        if deviation.abs() <= acceptable_err {
            return Ok(next_x);
        }
        current_x = next_x;
    }
    return Err(current_x);
}
