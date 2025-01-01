use rust_comp_geo::geometry::stroke::segment::reverse_segment;
use rust_comp_geo::geometry::stroke::arc::reverse_arc;
use rust_comp_geo::geometry::stroke::stroke::reverse_stroke;
use rust_comp_geo::geometry::types::types::{ Segment, Stroke, XY, Arc, SegmentBehavior, ArcBehavior };
use rust_comp_geo::geometry::point::point::points_equal;

#[cfg(test)]
mod segment_tests {

use super::*;

  #[test]
  fn test_reverse_segment() {
    let segment = Segment {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 }
    };

    let reversed = reverse_segment(&segment);
    assert!(points_equal(segment.get_p1(), reversed.get_p2(), None));
    assert!(points_equal(segment.get_p2(), reversed.get_p1(), None));
  }

  #[test]
  fn test_reverse_stroke_as_stroke_segment() {
    let segment: Stroke = Stroke::Segment(Segment {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 }
    });

    let reversed: Stroke = reverse_stroke(&segment);
    assert!(points_equal(segment.get_p1(), reversed.get_p2(), None));
    assert!(points_equal(segment.get_p2(), reversed.get_p1(), None));
  }

  #[test]
  fn test_reverse_arc() {
    let arc = Arc {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 },
      center: XY { x: 0.5, y: 0.5 },
      major: Some(true)
    };

    let reversed = reverse_arc(&arc);
    assert!(points_equal(arc.get_p1(), reversed.get_p2(), None));
    assert!(points_equal(arc.get_p2(), reversed.get_p1(), None));
    assert!(points_equal(arc.get_center().unwrap(), reversed.get_center().unwrap(), None));
    assert_eq!(arc.get_major(), reversed.get_major());
  }


  #[test]
  fn test_reverse_stroke_as_stroke_arc() {
    let arc: Stroke = Stroke::Arc(Arc {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 },
      center: XY { x: 0.5, y: 0.5 },
      major: Some(true)
    });

    let reversed: Stroke = reverse_stroke(&arc);
    assert!(points_equal(arc.get_p1(), reversed.get_p2(), None));
    assert!(points_equal(arc.get_p2(), reversed.get_p1(), None));
    assert!(points_equal(arc.get_center().unwrap(), reversed.get_center().unwrap(), None));
    assert_eq!(arc.get_major(), reversed.get_major());
  }
}