use crate::geometry::types::XY;

pub fn create_point(x: f64, y: f64) -> XY {
  XY { x, y }
}

pub fn p2p_dist (p1: XY, p2: XY) -> f64 {
  ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}
