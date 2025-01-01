use crate::geometry::types::stroke_types::StrokeBehavior;
use std::borrow::BorrowMut;
use std::hash::Hash;
use crate::geometry::stroke::stroke::reverse_stroke;
use crate::geometry::point::point::{bounding_box_from_points, points_equal};
use crate::quadtree::quadtree::QuadtreeProps;
use crate::quadtree::{quadtree::Quadtree, quadtree_point::QuadtreePoint};

pub struct UnscramblePathOpts<T: StrokeBehavior + Hash + Eq> {
    pub tolerance: Option<f64>,
    pub reverse: Option<fn(T) -> T>,
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

  let reverse = opts.unwrap().reverse.unwrap_or(|stroke| reverse_stroke(&stroke));
  // let tolerance = opts.unwrap().tolerance.unwrap_or(0.001);
  let tolerance = 0.001;

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

  points.iter().for_each(|point| {
    index.insert(&point[0]);
    index.insert(&point[1]);
  });

  let mut result: Vec<Vec<T>> = Vec::new();

  points.iter().for_each(|[start_point, end_point]| {
    // If this point has already been used in a path, then skip it.
    if (!start_point.data.used) {
      start_point.data.used = true;
      end_point.data.used = true;

      // First find all the connections to this point
      // in the forward direction along the path.
      let mut forward_path: Vec<T> = vec![start_point.data.stroke];
      let mut p2 = end_point;

      loop {
        let candidates = index.search(&p2, tolerance);

        let match_found = candidates.iter().find(|candidate| {
          !candidate.data.used && points_equal(p2.point, candidate.point, Some(tolerance))
        });

        match match_found {
          Some(matched_point) => {
            let mut match_point = points[match_found.unwrap().data.i];
            if match_found.unwrap().data.start {
              forward_path.push(match_found.unwrap().data.stroke);
              p2 = &match_point[1];
            } else {
              forward_path.push(reverse(match_found.unwrap().data.stroke));
              p2 = &match_point[0];
            }

            match_point[0].data.used = true;
            match_point[1].data.used = true;
          },
          None => break,
        }
      }

      // Next find all the connections to this point in the backwards
      // direction along the path.
      let mut backward_path: Vec<T> = vec![];
      let mut p1 = start_point;

      loop {
        let candidates = index.search(&p1, tolerance);

        let match_found = candidates.iter().find(|candidate| {
          !candidate.data.used && points_equal(p1.point, candidate.point, Some(tolerance))
        });

        match match_found {
          Some(matched_point) => {
            let mut match_point = points[match_found.unwrap().data.i];
            if match_found.unwrap().data.start {
              backward_path.push(reverse(match_found.unwrap().data.stroke));
              p1 = &match_point[1];
            } else {
              backward_path.push(match_found.unwrap().data.stroke);
              p1 = &match_point[0];
            }

            match_point[0].data.used = true;
            match_point[1].data.used = true;
          },
          None => break,
        }
      }

      result.push(backward_path.iter().rev().chain(forward_path.iter()).cloned().collect());
    }
  });

  return result;
}