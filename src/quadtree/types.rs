use crate::geometry::types::types::BoundingBox;

/**
 * Interface for objects that can be stored in a quadtree.
 */
pub trait QuadTreeObject<T> {
  fn get_data(&self) -> &T;
  fn in_node(&self, node: &BoundingBox) -> bool;
}

/**
 * Enum used to index the Quadtree nodes. Starts in
 * the bottom left quadrant and goes clockwise.
 */
pub enum Quadrant {
  BottomLeft = 0,
  TopLeft = 1,
  TopRight = 2,
  BottomRight = 3,
}

