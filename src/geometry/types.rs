use std::hash::{Hash, Hasher};

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

impl Hash for XY {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert f64 to bits for reliable hashing
        let x_bits = self.x.to_bits();
        let y_bits = self.y.to_bits();
        x_bits.hash(state);
        y_bits.hash(state);
    }
}

// You'll also need PartialEq and Eq
impl PartialEq for XY {
    fn eq(&self, other: &Self) -> bool {
        self.x.to_bits() == other.x.to_bits() && 
        self.y.to_bits() == other.y.to_bits()
    }
}

// Implementing Eq is required when implementing Hash
impl Eq for XY {} // This is safe because we're comparing bits

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

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: XY,
    pub radius: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}