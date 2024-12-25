use rust_comp_geo::geometry::point::{create_point, p2p_dist};
use rust_comp_geo::geometry::types::XY;

#[cfg(test)] // Only compiles when running tests
mod point_test {
  use super::*; // Import root scope

  #[test] // This attribute signals that this function is a test
  fn test_create_point() {
      let p: XY = create_point(4.0, 3.0);
      assert_eq!(p.x, 4.0);
      assert_eq!(p.y, 3.0);
  }

  #[test] // This attribute signals that this function is a test
  fn test_p2p_dist() {
      let p1: XY = XY {
        x: 3.0,
        y: 4.0,
      };    
      let p2: XY = XY {
        x: 0.0,
        y: 0.0,
      };
      let dist: f64 = p2p_dist(p1, p2);
      assert_eq!(dist, 5.0);
  }
}