use crate::geometry::types::stroke_types::ArcBehavior;

pub fn reverse_arc<T: ArcBehavior>(arc: &T) -> T where T: Clone {
  // Need to add condition for half circle to deal with major.
  let mut reversed_arc = arc.clone();
  reversed_arc.set_p1(arc.get_p2());
  reversed_arc.set_p2(arc.get_p1());

  return reversed_arc
}