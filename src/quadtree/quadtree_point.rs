use crate::quadtree::types::{ NodeGeometry, QuadTreeObject};
use crate::geometry::types::XY;

pub fn point_in_node(point: &XY, node: &NodeGeometry) -> bool {
  point.x >= node.x_min && point.x <= node.x_max && point.y >= node.y_min && point.y <= node.y_max
}

/**
 * Represents a 2D point in a quadtree.
 */
pub struct QuadtreePoint<U> {
  pub point: XY,
  pub data: U,
}

/**
 * Implementation of methods for QuadtreePoint.
 */
impl<U> QuadtreePoint<U> {
  pub fn new(point: XY, data: U) -> Self {
    QuadtreePoint { point, data }
  }
}

/**
 * Implementation of QuadTreeObject trait for QuadtreePoint.
 * This allows QuadtreePoint to be stored in a quadtree.
 */
impl<U> QuadTreeObject<U> for QuadtreePoint<U> {
  fn get_data(&self) -> &U {
    &self.data
  }

  fn in_node(&self, node: &NodeGeometry) -> bool {
    point_in_node(&self.point, node)
  }
}