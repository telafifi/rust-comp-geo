use serde::{Serialize, Deserialize};
use crate::geometry::types::types::XY;

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
  fn get_stroke(&self) -> Stroke;
}

pub trait AnnotatedStrokeBehavior<T> {
  fn get_data(&self) -> T;
  fn set_data(&mut self, data: T);
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
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

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
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

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
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
  fn get_stroke(&self) -> Stroke {
        self.clone()
  }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
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
    fn get_stroke(&self) -> Stroke {
        self.stroke.clone()
    }
}