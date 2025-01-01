use crate::geometry::types::SegmentBehavior;

pub fn reverse_segment<T: SegmentBehavior>(segment: &T) -> T where T: Clone {
    let mut reversed_segment = segment.clone();
    reversed_segment.set_p1(segment.get_p2());
    reversed_segment.set_p2(segment.get_p1());

    return reversed_segment
}