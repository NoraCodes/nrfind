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
