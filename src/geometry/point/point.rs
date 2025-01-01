use crate::geometry::angle::angle::correct_angle_signs;
use crate::geometry::types::types::{BoundingBox, XY};
use crate::utils::utils::close_equal;

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

pub fn angle_to_point(p2: XY) -> impl Fn(XY) -> f64 {
  move |p1| p2p_angle(p1, p2)
}

pub fn angle_from_point(p1: XY) -> impl Fn(XY) -> f64 {
  move |p2| p2p_angle(p1, p2)
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

/**
 * Determine if two points are equal to one another.
 */
pub fn points_equal(p1: XY, p2: XY, tolerance: Option<f64>) -> bool {
  close_equal(p1.x, p2.x, tolerance) && close_equal(p1.y, p2.y, tolerance)
}

/**
 * Curried version of points_equal.
 */
pub fn point_equals(p1: XY) -> impl Fn(XY) -> bool {
  move |p2: XY|  {
    points_equal(p1, p2, None)
  }
}

/**
 * Create a bounding box from a list of points.
 */
pub fn bounding_box_from_points(points: Vec<XY>) -> BoundingBox {
  let mut x_min = f64::INFINITY;
  let mut y_min = f64::INFINITY;
  let mut x_max = f64::NEG_INFINITY;
  let mut y_max = f64::NEG_INFINITY;

  for p in points.iter() {
    if p.x < x_min {
      x_min = p.x;
    }
    if p.y < y_min {
      y_min = p.y;
    }
    if p.x > x_max {
      x_max = p.x;
    }
    if p.y > y_max {
      y_max = p.y;
    }
  }

  BoundingBox {
    x_min,
    x_max,
    y_min,
    y_max,
  }
}