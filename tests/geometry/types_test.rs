use rust_comp_geo::geometry::types::{Arc, Segment, Stroke, StrokeBase, StrokeType, XY};

#[cfg(test)]
mod type_tests {

use super::*;

  #[test]
  fn test_serialize_stroke_base() {
    let stroke_base: StrokeBase = StrokeBase {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 },
      stroke_type: StrokeType::Segment
    };

    let serialized: String = serde_json::to_string(&stroke_base).unwrap();
    assert_eq!(serialized, r#"{"p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"type":"segment"}"#);
  }

  #[test]
  fn test_deserialize_stroke_base() {
    let serialized: &str = r#"{"p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"type":"arc"}"#;
    let deserialized: StrokeBase = serde_json::from_str(serialized).unwrap();
    assert_eq!(deserialized.p1.x, 0.0);
    assert_eq!(deserialized.p1.y, 0.0);
    assert_eq!(deserialized.p2.x, 1.0);
    assert_eq!(deserialized.p2.y, 1.0);
    assert_eq!(deserialized.stroke_type, StrokeType::Arc);
  }

  #[test]
  fn test_serialize_segment() {
    let segment: Stroke = Stroke::Segment(Segment {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 }
    });

    let serialized: String = serde_json::to_string(&segment).unwrap();
    assert_eq!(serialized, r#"{"type":"segment","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0}}"#);
  }

  #[test]
  fn test_serialize_arc() {
    let arc_major: Arc = Arc {
      p1: XY { x: 0.0, y: 0.0 },
      p2: XY { x: 1.0, y: 1.0 },
      center: XY { x: 0.5, y: 0.5 },
      major: Some(true)
    };

    // Cast to stroke.
    let stroke_major: Stroke = Stroke::Arc(arc_major);

    let serialized: String = serde_json::to_string(&stroke_major).unwrap();
    assert_eq!(serialized, r#"{"type":"arc","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"center":{"x":0.5,"y":0.5},"major":true}"#);

    let arc_minor: Arc = Arc {
      major: None,
      // Copy the rest of the fields from arc_major
      ..arc_major
    };

    // Cast to stroke.
    let stroke_minor: Stroke = Stroke::Arc(arc_minor);
    let serialized_minor: String = serde_json::to_string(&stroke_minor).unwrap();
    assert_eq!(serialized_minor, r#"{"type":"arc","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"center":{"x":0.5,"y":0.5}}"#);
  }

  // NEED TO ADD DESERIALIZATION TESTS AND THEN REMOVE BASE WORK
}