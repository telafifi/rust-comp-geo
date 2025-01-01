use std::hash::{Hash, Hasher};

use crate::quadtree::types::{ NodeGeometry, QuadTreeObject};
use crate::geometry::types::types::{Circle, XY};

/**
 * Check if a circle intersects with the given node.
 * This function calculates the intersection between a circle and a node in a quadtree.
 * It uses the rectangle-circle intersection test algorithm described in the following article:
 * https://yal.cc/rectangle-circle-intersection-test/
 */
pub fn circle_in_node(circle: &Circle, node: &NodeGeometry) -> bool {
  let NodeGeometry{ x_min, x_max, y_min, y_max } = *node;

  let clamped_x = f64::max(x_min, f64::min(circle.center.x, x_max));
  let delta_x = circle.center.x - clamped_x;

  let clamped_y = f64::max(y_min, f64::min(circle.center.y, y_max));
  let delta_y = circle.center.y - clamped_y;

  f64::powi(delta_x, 2) + f64::powi(delta_y, 2) <= f64::powi(circle.radius, 2)
}

#[derive(Debug, Clone)]
pub struct QuadtreeCircle<U> {
  pub center: XY,
  pub radius: f64,
  pub data: U,
}

/**
 * Implement Hashing and Equality for QuadtreeCircle, this is needed
 * to store QuadtreeCircle in a HashSet, and needs to be explicitly implemented
 * because the radius is a floating point number.
 */
impl <U: Hash> Hash for QuadtreeCircle<U> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.center.hash(state);
    self.data.hash(state);
    self.radius.to_bits().hash(state);
  }
}

impl<U: PartialEq> PartialEq for QuadtreeCircle<U> {
  fn eq(&self, other: &Self) -> bool {
      self.center == other.center &&
      self.data == other.data &&
      self.radius.to_bits() == other.radius.to_bits()
  }
}

impl<U: Eq> Eq for QuadtreeCircle<U> {}

/**
 * Implementation of methods for QuadtreeCircle.
 */
impl<U> QuadtreeCircle<U> {
  pub fn new(center: XY, radius: f64, data: U) -> Self {
    QuadtreeCircle { center, radius, data }
  }
}

/**
 * Implementation of QuadTreeObject trait for QuadtreeCircle.
 * This allows QuadtreeCircle to be stored in a quadtree.
 */
impl<U> QuadTreeObject<U> for QuadtreeCircle<U> {
  fn get_data(&self) -> &U {
    &self.data
  }

  fn in_node(&self, node: &NodeGeometry) -> bool {
    circle_in_node(&Circle { center: self.center, radius: self.radius }, node)
  }
}

