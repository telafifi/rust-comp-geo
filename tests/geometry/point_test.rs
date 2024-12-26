use rust_comp_geo::geometry::point::point::{create_point, p2p_dist, translate_point};
use rust_comp_geo::geometry::types::XY;
use rust_comp_geo::utils::utils::close_equal;

#[cfg(test)] // Only compiles when running tests
mod p2p_dist_tests {
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

#[cfg(test)]
mod translate_point_tests {

use super::*;

  #[test]
  fn translate_point_test() {
    let p: XY = XY {
      x: 5.0,
      y: 10.0,
    };
    let translation: XY = XY {
      x: 3.0,
      y: 4.0,
    };
    let translated = translate_point(translation)(p);
    assert!(close_equal(translated.x, 8.0, None));
    assert!(close_equal(translated.y, 14.0, None));
  }
}

#[cfg(test)]
mod p2p_angle_tests {
  use std::f64::consts::PI;

use rust_comp_geo::geometry::point::point::p2p_angle;

use super::*; // Import root scope

  #[test]
  fn p2p_angle_tests_per_quadrant() {
    let p1: XY = XY {
      x: 5.0,
      y: 10.0,
    };

    let vect: Vec<(XY, f64)> = vec![
      (XY { x: 5.0, y: 5.0 }, 0.25 * PI),
      (XY { x: -5.0, y: 5.0 }, 0.75 * PI),
      (XY { x: -5.0, y: -5.0 }, -0.75 * PI),
      (XY { x: 5.0, y: -5.0 }, -0.25 * PI),
    ];

    for (translation, expected_angle) in vect {
      let p2 = translate_point(translation)(p1);
      assert!(close_equal(p2p_angle(p1, p2), expected_angle, None));
    }
  }

}