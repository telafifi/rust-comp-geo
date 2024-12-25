use crate::geometry::types::Point;

pub fn create_point(x: f64, y: f64) -> Point {
  Point { x, y }
}

pub fn p2p_dist (p1: Point, p2: Point) -> f64 {
  ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}
