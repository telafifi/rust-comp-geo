use crate::geometry::angle::angle::correct_angle_signs;
use crate::geometry::types::XY;
// use crate::utils::utils::{close_equal, round};

pub fn create_point(x: f64, y: f64) -> XY {
  XY { x, y }
}

pub fn p2p_dist (p1: XY, p2: XY) -> f64 {
  ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn p2p_angle(p1: XY, p2: XY) -> f64 {
  let delta_y = p2.y - p1.y;
  let delta_x = p2.x - p1.x;

  correct_angle_signs(delta_y.atan2(delta_x))
}

/**
 * Create a new point translated by the offset X and Y values provided.
 */
pub fn translate_point(translation: XY) -> impl Fn(XY) -> XY {
  move |p: XY| XY {
    x: translation.x + p.x,
    y: translation.y + p.y
  }
}