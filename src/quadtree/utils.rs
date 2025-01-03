use crate::geometry::types::types::{BoundingBox, XY};
use crate::geometry::types::stroke_types::Segment;

/**
 * Offset the bounds of a node by the given amount making the node larger.
 */
pub fn offset_node_bounds(node: &BoundingBox, distance: f64)-> BoundingBox {
  BoundingBox {
    x_min: node.x_min - distance,
    x_max: node.x_max + distance,
    y_min: node.y_min - distance,
    y_max: node.y_max + distance,
  }
}

/**
 * Returns the four corner points of a node.
 */
pub fn get_node_points(node: &BoundingBox) -> Vec<XY> {
  vec![
    XY { x: node.x_min, y: node.y_min },
    XY { x: node.x_min, y: node.y_max },
    XY { x: node.x_max, y: node.y_max },
    XY { x: node.x_max, y: node.y_min },
  ]
}

/**
 * Returns an array of segments representing the edges of a node.
 */
pub fn get_node_edges(points: &Vec<XY>) -> Vec<Segment> {
  let mut edges: Vec<Segment> = Vec::new();
  for i in 0..points.len() {
    let p1 = points[i];
    let p2 = points[(i + 1) % points.len()];
    edges.push(Segment { p1, p2 });
  }
  edges
}