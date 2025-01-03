use crate::geometry::types::stroke_types::{AnnotatedStroke, StrokeBehavior, StrokeType};
use crate::geometry::stroke::segment::reverse_segment;
use crate::geometry::stroke::arc::reverse_arc;

pub fn reverse_stroke<T: StrokeBehavior>(stroke: &T) -> T where T: Clone {
    match stroke.get_type() {
        StrokeType::Segment => {
            return reverse_segment(stroke)
        },
        StrokeType::Arc => {
          return reverse_arc(stroke)
        }
    }
}

pub fn stroke_with_new_data<T: StrokeBehavior, U>(stroke: T, data: U) -> AnnotatedStroke<U> {
    let new_stroke = stroke.get_stroke();
    AnnotatedStroke {
        stroke: new_stroke,
        data,
    }
}