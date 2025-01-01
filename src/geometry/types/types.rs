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

pub enum StrokeType {
    Segment,
    Arc,
}

pub trait SegmentBehavior {
    fn get_p1(&self) -> XY;
    fn get_p2(&self) -> XY;
    fn set_p1(&mut self, p: XY);
    fn set_p2(&mut self, p: XY);
}

pub trait ArcBehavior: SegmentBehavior {
    fn get_center(&self) -> Option<XY>;
    fn get_major(&self) -> Option<bool>;
    fn set_center(&mut self, p: XY);
    fn set_major(&mut self, major: bool);
}

pub trait StrokeBehavior: SegmentBehavior + ArcBehavior {
    fn get_type(&self) -> StrokeType;
}

pub trait AnnotatedStrokeBehavior<T> {
    fn get_data(&self) -> T;
    fn set_data(&mut self, data: T);
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(rename = "segment")]
pub struct Segment {
    pub p1: XY,
    pub p2: XY,
}

impl SegmentBehavior for Segment {
    fn get_p1(&self) -> XY {
        self.p1
    }

    fn get_p2(&self) -> XY {
        self.p2
    }

    fn set_p1(&mut self, p: XY) {
        self.p1 = p;
    }

    fn set_p2(&mut self, p: XY) {
        self.p2 = p;
    }
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

impl SegmentBehavior for Arc {
    fn get_p1(&self) -> XY {
        self.p1
    }

    fn get_p2(&self) -> XY {
        self.p2
    }

    fn set_p1(&mut self, p: XY) {
        self.p1 = p;
    }

    fn set_p2(&mut self, p: XY) {
        self.p2 = p;
    }
}

impl ArcBehavior for Arc {
    fn get_center(&self) -> Option<XY> {
        Some(self.center)
    }

    fn get_major(&self) -> Option<bool> {
        self.major
    }

    fn set_center(&mut self, p: XY) {
        self.center = p;
    }

    fn set_major(&mut self, major: bool) {
        self.major = Some(major);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Stroke {
    #[serde(rename = "segment")]
    Segment(Segment),
    #[serde(rename = "arc")]
    Arc(Arc),
}

impl SegmentBehavior for Stroke {
    fn get_p1(&self) -> XY {
        match self {
            Stroke::Segment(segment) => segment.get_p1(),
            Stroke::Arc(arc) => arc.get_p1(),
        }
    }

    fn get_p2(&self) -> XY {
        match self {
            Stroke::Segment(segment) => segment.get_p2(),
            Stroke::Arc(arc) => arc.get_p2(),
        }
    }

    fn set_p1(&mut self, p: XY) {
        match self {
            Stroke::Segment(segment) => segment.set_p1(p),
            Stroke::Arc(arc) => arc.set_p1(p),
        }
    }

    fn set_p2(&mut self, p: XY) {
        match self {
            Stroke::Segment(segment) => segment.set_p2(p),
            Stroke::Arc(arc) => arc.set_p2(p),
        }
    }
}

impl ArcBehavior for Stroke {
    fn get_center(&self) -> Option<XY> {
        match self {
            Stroke::Segment(_) => None,
            Stroke::Arc(arc) => arc.get_center(),
        }
    }

    fn get_major(&self) -> Option<bool> {
        match self {
            Stroke::Segment(_) => None,
            Stroke::Arc(arc) => arc.get_major(),
        }
    }

    fn set_center(&mut self, p: XY) {
        match self {
            Stroke::Segment(_) => {},
            Stroke::Arc(arc) => arc.set_center(p),
        }
    }

    fn set_major(&mut self, major: bool) {
        match self {
            Stroke::Segment(_) => {},
            Stroke::Arc(arc) => arc.set_major(major),
        }
    }
}

impl StrokeBehavior for Stroke {
    fn get_type(&self) -> StrokeType {
        match self {
            Stroke::Segment(_) => StrokeType::Segment,
            Stroke::Arc(_) => StrokeType::Arc,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct AnnotatedStroke<T> {
    #[serde(flatten)]
    pub stroke: Stroke,
    pub data: T,
}

impl<T> SegmentBehavior for AnnotatedStroke<T> {
    fn get_p1(&self) -> XY {
        self.stroke.get_p1()
    }

    fn get_p2(&self) -> XY {
        self.stroke.get_p2()
    }

    fn set_p1(&mut self, p: XY) {
        self.stroke.set_p1(p);
    }

    fn set_p2(&mut self, p: XY) {
        self.stroke.set_p2(p);
    }
}

impl<T> ArcBehavior for AnnotatedStroke<T> {
    fn get_center(&self) -> Option<XY> {
        self.stroke.get_center()
    }

    fn get_major(&self) -> Option<bool> {
        self.stroke.get_major()
    }

    fn set_center(&mut self, p: XY) {
        self.stroke.set_center(p);
    }

    fn set_major(&mut self, major: bool) {
        self.stroke.set_major(major);
    }
}

impl<T> StrokeBehavior for AnnotatedStroke<T> {
    fn get_type(&self) -> StrokeType {
        self.stroke.get_type()
    }
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