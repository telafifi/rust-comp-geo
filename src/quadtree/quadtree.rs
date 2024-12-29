use crate::quadtree::utils::offset_node_bounds;
use crate::quadtree::types::{NodeGeometry, QuadTreeObject};

pub struct QuadtreeProps {
  pub bounds: NodeGeometry,
  pub max_objects: usize,
  pub max_levels: usize,
}

impl Default for QuadtreeProps {
  fn default() -> Self {
    QuadtreeProps {
      bounds: NodeGeometry::default(),
      max_objects: 10,
      max_levels: 4,
    }
  }
}

// pub struct Quadtree<T