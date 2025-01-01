use rust_comp_geo::geometry::stroke::segment::reverse_segment;
use rust_comp_geo::geometry::types::{ Segment, Stroke, XY };

#[cfg(test)]
mod segment_tests {
  use super::*;

  #[test]
  fn test_reverse_segment_as_stroke() {
    let segment: Stroke = Stroke::Segment(Segment {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 }
    });

    let reversed: Stroke = reverse_segment(&segment);
    match reversed {
      Stroke::Segment(segment) => {
        assert_eq!(segment.p1.x, 1.0);
        assert_eq!(segment.p1.y, 1.0);
        assert_eq!(segment.p2.x, 0.0);
        assert_eq!(segment.p2.y, 0.0);
      },
      _ => panic!("Expected segment")
    }
  }
}