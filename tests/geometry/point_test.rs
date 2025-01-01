use rust_comp_geo::geometry::point::point::{
  bounding_box_from_points, 
  create_point, 
  p2p_angle, 
  p2p_dist, 
  points_equal, 
  point_equals, 
  translate_point
};
use rust_comp_geo::geometry::types::types::{ BoundingBox, XY };
use rust_comp_geo::utils::utils::close_equal;
use std::f64::consts::PI;

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

#[cfg(test)]
mod points_equal_tests {
  use super::*;

  #[test]
  fn test_points_equal() {
    let p1: XY = XY {
      x: 0.0,
      y: 10.0,
    };

    let vect: Vec<(XY, bool)> = vec![
      (XY { x: -0.0000001, y: 10.0 }, true),
      (XY { x: 0.0, y: 10.0000000001 }, true),
      (XY { x: 0.0000001, y: 10.000001 }, true),
      (XY { x: 0.1, y: 10.0001 }, false),
    ];

    for (p2, expected) in vect {
      assert_eq!(point_equals(p1)(p2), expected);
      assert_eq!(points_equal(p1, p2, None), expected);
    }
  }
}

#[cfg(test)]
mod bounding_box_from_points_tests {
  use super::*;

  #[test]
  fn bounding_box_from_points_test() {
    let points: Vec<XY> = vec![
      XY { x: 5.0, y: 5.0 },
      XY { x: 0.0, y: 0.0 },
      XY { x: 5.0, y: 10.0 },
      XY { x: 10.0, y: 10.0 },
    ];

    let bb: BoundingBox = bounding_box_from_points(&points);
    assert_eq!(bb.x_min, 0.0);
    assert_eq!(bb.y_min, 0.0);
    assert_eq!(bb.x_max, 10.0);
    assert_eq!(bb.y_max, 10.0);
  }
}