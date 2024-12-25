pub fn close_equal(a: f64, b: f64, tolerance: Option<f64>) -> bool {
    (a - b).abs() < tolerance.unwrap_or(0.000001)
}
