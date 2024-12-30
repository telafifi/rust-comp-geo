/**
 * Interface for geometry of a Quadtree node.
 */
#[derive(Default, Copy, Clone)]
pub struct NodeGeometry {
  pub x_min: f64,
  pub x_max: f64,
  pub y_min: f64,
  pub y_max: f64,
}

/**
 * Interface for objects that can be stored in a quadtree.
 */
pub trait QuadTreeObject<T> {
  fn get_data(&self) -> &T;
  fn in_node(&self, node: &NodeGeometry) -> bool;
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

