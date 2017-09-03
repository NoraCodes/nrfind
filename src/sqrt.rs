use std::ops::{Sub, Mul, Div, Add};
use num::Float;
use super::{find_root, NumericSolutionResult};

/// Find the square root of a given `radicand` to within a given `acceptable_err`
/// by the Newton-Rahpson method, using the initial guess `x0`. If, after
/// `max_iterations`, the error is not less than the give `acceptable_err`, 
/// `None` is returned.
///
/// # Examples
///
/// ```
/// use nrfind::find_sqrt;
///
/// let precision = 0.1;
/// let iterations = 100;
/// let radicand = 25.6;
/// let guess = 10.0;
///
/// // This will produce a close approximation of the actual square root
/// let value = find_sqrt(radicand, guess, precision, iterations).unwrap();
///
/// let actual: f64 = 5.0497;
/// let difference = (value - actual).abs();
///
/// assert!(difference < precision);
/// 
/// // This will fail, as there is no real solution.
/// let value = find_sqrt(-radicand, guess, precision, iterations);
/// value.unwrap_err();
/// ```
///
pub fn find_sqrt<N>(radicand: N, x0: N, 
                    acceptable_err: N, max_iterations: i32)
                    -> NumericSolutionResult<N> 
                    where N: Float + Div + Sub + Mul + Add + PartialOrd {


    find_root(&|x: N| x.powi(2) - radicand, 
              // x + x = 2x, this lets us forgoe the From<f64> trait bound.
              &|x: N| x + x, 
              x0, 
              acceptable_err, 
              max_iterations)
}
