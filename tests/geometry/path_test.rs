use rust_comp_geo::geometry::path::path::unscramble_path;
use rust_comp_geo::geometry::types::types:: XY;
use rust_comp_geo::geometry::types::stroke_types::{ Stroke, Segment, Arc };

#[cfg(test)]
mod unscramble_path_tests {
  use super::*;

  #[test]
  fn test_unscramble_path() {
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