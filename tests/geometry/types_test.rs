use rust_comp_geo::geometry::types::{StrokeBase, StrokeType, XY};

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
}