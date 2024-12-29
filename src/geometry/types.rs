use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Vector2D {
    pub i: f64,
    pub j: f64,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Vector3D {
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]

pub enum StrokeType {
    Segment,
    Arc
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct StrokeBase {
    pub p1: XY,
    pub p2: XY,
    // Need to test serialization and deserialization from json
    #[serde(rename="type")]
    pub stroke_type: StrokeType
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(rename = "segment")]
pub struct Segment {
    pub p1: XY,
    pub p2: XY,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(rename = "arc")]
pub struct Arc {
    pub p1: XY,
    pub p2: XY,
    pub center: XY,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<bool>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Stroke {
    #[serde(rename = "segment")]
    Segment(Segment),
    #[serde(rename = "arc")]
    Arc(Arc),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct AnnotatedStroke<T> {
    #[serde(flatten)]
    pub stroke: Stroke,
    pub data: T,
}