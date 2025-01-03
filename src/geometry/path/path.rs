use crate::geometry::types::stroke_types::StrokeBehavior;
use std::hash::Hash;
use crate::geometry::stroke::stroke::reverse_stroke;
use crate::geometry::point::point::{bounding_box_from_points, points_equal};
use crate::quadtree::quadtree::QuadtreeProps;
use crate::quadtree::{quadtree::Quadtree, quadtree_point::QuadtreePoint};

pub struct UnscramblePathOpts<T: StrokeBehavior + Hash + Eq> {
    pub tolerance: Option<f64>,
    pub reverse: Option<fn(&T) -> T>,
}

pub fn unscramble_path<T: StrokeBehavior + Clone + Hash + Eq>(
  strokes: Vec<T>, 
  opts: Option<UnscramblePathOpts<T>>
) -> Vec<Vec<T>> {
  #[derive(Hash, Eq, PartialEq, Clone)]
  struct IndexedData<T> {
    stroke: T,
    start: bool,
    used: bool,
    i: usize,
  }

  // Default values
  let reverse = match &opts {
    Some(opt) => opt.reverse.unwrap_or(reverse_stroke),
    None => reverse_stroke
  };
  let tolerance = match &opts {
    Some(opt) => opt.tolerance.unwrap_or(0.001),
    None => 0.001
  };

  let stroke_extremes = strokes.iter().map(|stroke| [stroke.get_p1(), stroke.get_p2()]).flatten().collect::<Vec<_>>();
  let bounds = bounding_box_from_points(&stroke_extremes);

  let mut index: Quadtree<QuadtreePoint<IndexedData<T>>, IndexedData<T>> = Quadtree::new(QuadtreeProps{
    bounds,
    max_objects: 10,
    max_levels: 4,
  }, 0);

  let mut points: Vec<[QuadtreePoint<IndexedData<T>>; 2]> = strokes.iter().enumerate().map(|(i, stroke)| [
    QuadtreePoint::new(stroke.get_p1(), IndexedData { stroke: stroke.clone(), start: true, used: false, i }),
    QuadtreePoint::new(stroke.get_p2(), IndexedData { stroke: stroke.clone(), start: false, used: false, i }),
  ]).collect::<Vec<[QuadtreePoint<IndexedData<T>>; 2]>>();

  points.iter().for_each(|[start_point, end_point]| {
    index.insert(start_point);
    index.insert(end_point);
  });

  let mut result: Vec<Vec<T>> = Vec::new();

  for i in 0..points.len() {
    if !points[i][0].data.used {
      points[i][0].data.used = true;
      points[i][1].data.used = true;

      // First find all the connections to this point
      // in the forward direction along the path.
      let mut forward_path: Vec<T> = vec![points[i][0].data.stroke.clone()];
      let mut p2 = &points[i][1];

      loop {
        let candidates = index.search(p2, tolerance);

        let match_found = candidates.iter().find(|candidate| {
          let index = candidate.data.i;
          let sub_index = if candidate.data.start { 0 } else { 1 };
          // Need to use the original point data to compare, as that is the owner that is having data mutated
          // and we need to ensure that the same point is not used twice.
          let candidate_point = &points[index][sub_index];
          !candidate_point.data.used && points_equal(p2.point, candidate.point, Some(tolerance))
        });

        match match_found {
          Some(matched_point) => {
            let point_idx = matched_point.data.i;
            let next_sub_index = if matched_point.data.start {
                forward_path.push(matched_point.data.stroke.clone());
                1
            } else {
                forward_path.push(reverse(&matched_point.data.stroke.clone()));
                0
            };

            points[point_idx][0].data.used = true;
            points[point_idx][1].data.used = true;

            // Update p2 with a new QuadtreePoint using the stored point data
            p2 = &points[point_idx][next_sub_index];
          },
          None => break,
        }
      }

      // Next find all the connections to this point in the backwards
      // direction along the path.
      let mut backward_path: Vec<T> = vec![];
      let mut p1 = &points[i][0];

      loop {
        let candidates = index.search(p1, tolerance);

        let match_found = candidates.iter().find(|candidate| {
          let index = candidate.data.i;
          let sub_index = if candidate.data.start { 0 } else { 1 };
          let candidate_point = &points[index][sub_index];
          !candidate_point.data.used && points_equal(p1.point, candidate.point, Some(tolerance))
        });

        match match_found {
          Some(matched_point) => {
            let point_idx = matched_point.data.i;

            let next_sub_index = if matched_point.data.start {
              backward_path.push(reverse(&matched_point.data.stroke.clone()));
              1
            } else {
              backward_path.push(matched_point.data.stroke.clone());
              0
            };

            points[point_idx][0].data.used = true;
            points[point_idx][1].data.used = true;

            p1 = &points[point_idx][next_sub_index];
          },
          None => break,
        }
      }

      result.push(backward_path.iter().rev().chain(forward_path.iter()).cloned().collect());
    }
  }

  return result;
}