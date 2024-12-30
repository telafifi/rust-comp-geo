use std::marker::PhantomData;
use std::hash::Hash;

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

pub struct Quadtree<T, U> where T: QuadTreeObject<U> {
  pub bounds: NodeGeometry,
  pub max_objects: usize,
  pub max_levels: usize,
  pub level: usize,
  pub objects: Vec<T>,
  pub nodes: Vec<Quadtree<T, U>>,
  _phantom: PhantomData<U>,  // Add PhantomData to use U non-recursively
}

impl<T: Clone, U> Quadtree<T, U> 
where T: QuadTreeObject<U> + Hash + Eq {
  pub fn new(props: QuadtreeProps, level: usize) -> Quadtree<T, U> {
    Quadtree {
      bounds: props.bounds,
      max_objects: props.max_objects,
      max_levels: props.max_levels,
      level,
      objects: Vec::new(),
      nodes: Vec::new(),
      _phantom: PhantomData,
    }
  }

  pub fn split(&mut self) -> () {
    let level = self.level + 1;
    let NodeGeometry { x_min, x_max, y_min, y_max } = self.bounds;

    let x_mid = (x_min + x_max) / 2.0;
    let y_mid = (y_min + y_max) / 2.0;

    // Compute the bounds of the subnodes
    // which are the quadrants of the current node.
    // Note: this is done in a way that doesn't leave any
    // gaps between the quadrants due to rounding error, so
    // the inNode() implementations don't need to account for
    // rounding error gaps between the quadrants.
    let bounds: Vec<NodeGeometry> = vec![
      NodeGeometry {
        x_min,
        x_max: x_mid,
        y_min,
        y_max: y_mid,
      },
      NodeGeometry {
        x_min,
        x_max: x_mid,
        y_min: y_mid,
        y_max,
      },
      NodeGeometry {
        x_min: x_mid,
        x_max,
        y_min: y_mid,
        y_max,
      },
      NodeGeometry {
        x_min: x_mid,
        x_max,
        y_min,
        y_max: y_mid,
      },
    ];

    self.nodes = bounds.iter()
    .map(|bound| {
        Quadtree::new(
            QuadtreeProps {
                bounds: *bound,
                max_objects: self.max_objects,
                max_levels: self.max_levels,
            },
            level
        )
    })
    .collect();

  }

  pub fn insert(&mut self, obj: &T) -> &mut Self {
    // If we have sub-nodes, call insert on matching subnodes.
    if !self.nodes.is_empty() {
      for node in &mut self.nodes {
          if obj.in_node(&node.bounds) {
              node.insert(obj);
          }
      }
      return self;
  }

    // Otherwise, store the object in this node.
    self.objects.push(obj.clone());

    // If we have too many objects in this node, and we haven't
    // reached the maximum depth, split the node.
    if self.objects.len() > self.max_objects && self.level < self.max_levels {
      self.split();

      // Move all objects to their corresponding subnodes.
      for obj in self.objects.iter() {
        for node in self.nodes.iter_mut() {
          if obj.in_node(&node.bounds) {
            node.insert(obj);
          }
        }
      }

      // Clean up the current node.
      self.objects.clear();
    }

    return self;
  }

  pub fn search<V>(&self, obj: &V, distance: f64) -> Vec<&T> 
  where V: QuadTreeObject<U> {
    let mut result: Vec<&T> = self.objects.iter().collect();

    // If we have sub-nodes, retrieve objects from them.
    for node in &self.nodes {
      let offset = offset_node_bounds(&node.bounds, distance);
      if obj.in_node(&offset) {
        result.extend(node.search(obj, distance));
      }
    }

    if self.level == 0 {
      // Convert to HashSet and back to remove duplicates
      let unique: std::collections::HashSet<&T> = result.into_iter().collect();
      result = unique.into_iter().collect();
    }

    result
  }

  /**
   * Clear the quadtree.
   */
  pub fn clear(&mut self) -> &mut Self {
    self.objects.clear();
    for node in self.nodes.iter_mut() {
      node.clear();
    }
    self.nodes.clear();
    return self
  }
}