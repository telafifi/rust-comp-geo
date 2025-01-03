use rust_comp_geo::geometry::point::point::points_equal;
use rust_comp_geo::geometry::path::path::unscramble_path;
use rust_comp_geo::geometry::types::types:: XY;
use rust_comp_geo::geometry::types::stroke_types::{ Stroke, Segment, Arc };

#[cfg(test)]
mod unscramble_path_tests {
  use rust_comp_geo::geometry::types::stroke_types::SegmentBehavior;

use super::*;

  #[test]
  fn test_unscramble_path_correctly_extracts_contiguous() {
    let path: Vec<Stroke> = vec![
      Stroke::Segment(Segment {
        p1: XY { x: 7.0, y: 5.0 },
        p2: XY { x: 0.0, y: 5.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 10.0, y: 0.0 },
        p2: XY { x: 10.0, y: 2.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 0.0, y: 0.0 },
        p2: XY { x: 10.0, y: 0.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 0.0, y: 5.0 },
        p2: XY { x: 0.0, y: 1.0 },
      }),
      Stroke::Arc(Arc {
        center: XY { x: 7.0, y: 2.0 },
        p1: XY { x: 10.0, y: 2.0 },
        p2: XY { x: 7.0, y: 5.0 },
        major: None,
      }),
    ];

    let unscrambled = unscramble_path(path, None);
    assert_eq!(unscrambled.len(), 1);

    let first_path = &unscrambled[0];
    assert_eq!(first_path.len(), 5);
    let first = &first_path[0].get_p1();
    let second = &first_path[1].get_p1();
    let third = &first_path[2].get_p1();
    let fourth = &first_path[3].get_p1();
    let fifth = &first_path[4].get_p1();
    assert!(points_equal(*first, XY { x: 0.0, y: 0.0 }, None));
    assert!(points_equal(*second, XY { x: 10.0, y: 0.0 }, None));
    assert!(points_equal(*third, XY { x: 10.0, y: 2.0 }, None));
    assert!(points_equal(*fourth, XY { x: 7.0, y: 5.0 }, None));
    assert!(points_equal(*fifth, XY { x: 0.0, y: 5.0 }, None));
  }

  #[test]
  fn test_unscramble_path_correctly_extracts_contiguous_with_reversal() {
    let path: Vec<Stroke> = vec![
      Stroke::Segment(Segment {
        p1: XY { x: 7.0, y: 5.0 },
        p2: XY { x: 0.0, y: 5.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 10.0, y: 2.0 },
        p2: XY { x: 10.0, y: 0.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 0.0, y: 0.0 },
        p2: XY { x: 10.0, y: 0.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 0.0, y: 1.0 },
        p2: XY { x: 0.0, y: 5.0 },
      }),
      Stroke::Arc(Arc {
        center: XY { x: 7.0, y: 2.0 },
        p1: XY { x: 7.0, y: 5.0 },
        p2: XY { x: 10.0, y: 2.0 },
        major: None,
      }),
    ];

    let unscrambled = unscramble_path(path, None);
    assert_eq!(unscrambled.len(), 1);

    let first_path = &unscrambled[0];
    assert_eq!(first_path.len(), 5);
    let first = &first_path[0].get_p1();
    let second = &first_path[1].get_p1();
    let third = &first_path[2].get_p1();
    let fourth = &first_path[3].get_p1();
    let fifth = &first_path[4].get_p1();
    assert!(points_equal(*first, XY { x: 0.0, y: 0.0 }, None));
    assert!(points_equal(*second, XY { x: 10.0, y: 0.0 }, None));
    assert!(points_equal(*third, XY { x: 10.0, y: 2.0 }, None));
    assert!(points_equal(*fourth, XY { x: 7.0, y: 5.0 }, None));
    assert!(points_equal(*fifth, XY { x: 0.0, y: 5.0 }, None));
  }

  #[test]
  fn test_unscramble_path_multiple_path_output() {
    let path: Vec<Stroke> = vec![
      Stroke::Segment(Segment {
        p1: XY { x: 7.0, y: 5.0 },
        p2: XY { x: 0.0, y: 5.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 0.0, y: 0.0 },
        p2: XY { x: 10.0, y: 0.0 },
      }),
      Stroke::Segment(Segment {
        p1: XY { x: 0.0, y: 1.0 },
        p2: XY { x: 0.0, y: 5.0 },
      }),
      Stroke::Arc(Arc {
        center: XY { x: 7.0, y: 2.0 },
        p1: XY { x: 7.0, y: 5.0 },
        p2: XY { x: 10.0, y: 2.0 },
        major: Some(false),
      }),
    ];

    let unscrambled = unscramble_path(path, None);
    assert_eq!(unscrambled.len(), 2);
  }
}