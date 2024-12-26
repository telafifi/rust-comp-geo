/**
 * Check if two numbers are close to each other, under a certain tolerance.
 */
pub fn close_equal(a: f64, b: f64, tolerance: Option<f64>) -> bool {
    (a - b).abs() < tolerance.unwrap_or(0.000001)
}

/**
 * Round a number to n decimal places.
 */
pub fn round_to_n_decimals(x: f64, n: u32) -> f64 {
    let factor = 10_f64.powi(n as i32);
    (x * factor).round() / factor
}

/**
 * Round a number to 6 decimal places.
 */
pub fn round(x: f64) -> f64 {
    round_to_n_decimals(x, 6)
}