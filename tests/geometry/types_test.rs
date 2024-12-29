use serde::{Serialize, Deserialize};
use rust_comp_geo::geometry::types::{Arc, AnnotatedStroke, Segment, Stroke, StrokeBase, StrokeType, XY};

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

  #[test]
  fn test_deserialize_segment() {
    let serialized: &str = r#"{"type":"segment","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0}}"#;
    let deserialized: Stroke = serde_json::from_str(serialized).unwrap();
    match deserialized {
      Stroke::Segment(segment) => {
        assert_eq!(segment.p1.x, 0.0);
        assert_eq!(segment.p1.y, 0.0);
        assert_eq!(segment.p2.x, 1.0);
        assert_eq!(segment.p2.y, 1.0);
      },
      _ => panic!("Expected segment")
    }
  }

  #[test]
  fn test_deserialize_arc() {
    let serialized_major: &str = r#"{"type":"arc","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"center":{"x":0.5,"y":0.5},"major":true}"#;
    let deserialized_major: Stroke = serde_json::from_str(serialized_major).unwrap();
    match deserialized_major {
      Stroke::Arc(arc) => {
        assert_eq!(arc.p1.x, 0.0);
        assert_eq!(arc.p1.y, 0.0);
        assert_eq!(arc.p2.x, 1.0);
        assert_eq!(arc.p2.y, 1.0);
        assert_eq!(arc.center.x, 0.5);
        assert_eq!(arc.center.y, 0.5);
        assert_eq!(arc.major, Some(true));
      },
      _ => panic!("Expected arc")
    }

    let serialized_minor: &str = r#"{"type":"arc","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"center":{"x":0.5,"y":0.5}}"#;
    let deserialized_minor: Stroke = serde_json::from_str(serialized_minor).unwrap();
    match deserialized_minor {
      Stroke::Arc(arc) => {
        assert_eq!(arc.p1.x, 0.0);
        assert_eq!(arc.p1.y, 0.0);
        assert_eq!(arc.p2.x, 1.0);
        assert_eq!(arc.p2.y, 1.0);
        assert_eq!(arc.center.x, 0.5);
        assert_eq!(arc.center.y, 0.5);
        assert_eq!(arc.major, None);
      },
      _ => panic!("Expected arc")
    }
  }

  #[test]
  fn test_serialize_annotated_stroke() {
    #[derive(Clone, Serialize, Deserialize, Debug)]
    struct StrokeData {
      pub name: String,
      pub value: f64
    }
    
    let annotated_stroke: AnnotatedStroke<StrokeData> = AnnotatedStroke {
      stroke: Stroke::Arc(Arc {
        p1: XY { x: 0.0, y: 0.0 },
        p2: XY { x: 1.0, y: 1.0 },
        center: XY { x: 0.5, y: 0.5 },
        major: Some(true)
      }),
      data: StrokeData {
        name: "strokeName".to_string(),
        value: 3.0,
      }
    };

    let serialized: String = serde_json::to_string(&annotated_stroke).unwrap();
    assert_eq!(serialized, r#"{"type":"arc","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"center":{"x":0.5,"y":0.5},"major":true,"data":{"name":"strokeName","value":3.0}}"#);
  }

  #[test]
  fn test_deserialize_annotated_stroke() {
    #[derive(Clone, Serialize, Deserialize, Debug)]
    struct StrokeData {
      pub name: String,
      pub value: f64
    }

    let serialized: &str = r#"{"type":"arc","p1":{"x":0.0,"y":0.0},"p2":{"x":1.0,"y":1.0},"center":{"x":0.5,"y":0.5},"major":true,"data":{"name":"strokeName","value":3.0}}"#;
    let deserialized: AnnotatedStroke<StrokeData> = serde_json::from_str(serialized).unwrap();
    match deserialized {
      AnnotatedStroke { stroke, data } => {
        match stroke {
          Stroke::Arc(arc) => {
            assert_eq!(arc.p1.x, 0.0);
            assert_eq!(arc.p1.y, 0.0);
            assert_eq!(arc.p2.x, 1.0);
            assert_eq!(arc.p2.y, 1.0);
            assert_eq!(arc.center.x, 0.5);
            assert_eq!(arc.center.y, 0.5);
            assert_eq!(arc.major, Some(true));
          },
          _ => panic!("Expected arc")
        }
        assert_eq!(data.name, "strokeName");
        assert_eq!(data.value, 3.0);
      }
    }
  }
}