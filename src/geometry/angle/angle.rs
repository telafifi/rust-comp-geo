use std::f64::consts::PI;
use crate::utils::utils::close_equal;

/**
 * Verify that a number is not negative zero, if so then replace it.
 */
pub fn no_negative_zero(x: f64) -> f64 {
    if x == 0.0 { 0.0 } else { x }
}

/**
 * Ensure that no negative π or negative zero.
 */
pub fn correct_angle_signs(x: f64) -> f64 {
  if close_equal(x, -1.0 * PI, None) {
    x + PI
  } else {
    no_negative_zero(x)
  }
}