/**
 * Interface for geometry of a Quadtree node.
 */
#[derive(Default)]
pub struct NodeGeometry {
  pub x_min: f64,
  pub x_max: f64,
  pub y_min: f64,
  pub y_max: f64,
}

/**
 * Interface for objects that can be stored in a quadtree.
 */
pub struct QuadTreeObject<U> {
  /**
   * Arbitrary data associated with the object.
   */
  pub data: U,

  /**
   * Check if the object is within a node.
   */
  pub in_node: fn(node: &NodeGeometry) -> bool,
}

/**
 * Enum used to index the Quadtree nodes. Starts in
 * the bottom left quadrant and goes clockwise.
 */
pub enum Quadrant {
  BottomLeft,
  TopLeft,
  TopRight,
  BottomRight,
}

