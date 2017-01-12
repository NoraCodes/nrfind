use std::ops::{Sub, Mul, Div};
use num::Float;
use super::find_root;

/// Find the square root of a given `radicand` to within a given `acceptable_err`
/// by the Newton-Rahpson method, initial guess `x0`, and give up after 
/// `max_iterations` (in which case `None` is returned).
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
/// let value = find_sqrt(&radicand, &guess, &precision, iterations).unwrap();
///
/// let actual = 5.0497;
/// let difference = (value - actual).abs();
///
/// assert!(difference < precision);
/// 
/// // This will fail, as there is no real solution.
/// let value = find_sqrt(&(-radicand), &guess, &precision, iterations);
/// assert!(value.is_none());
/// ```
///
pub fn find_sqrt<N>(radicand: &N, x0: &N, 
                    acceptable_err: &N, max_iterations: i32)
                    -> Option<N> 
                    where N: Float + Div + Sub + Mul + PartialOrd + From<f64> {


    find_root(&|x: N| x.powi(2) - *radicand, 
              &|x: N| x * 2.0.into(), 
              x0, 
              acceptable_err, 
              max_iterations)
}
