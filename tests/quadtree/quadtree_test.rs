use rust_comp_geo::quadtree::quadtree::Quadtree;
use rust_comp_geo::quadtree::quadtree_point::QuadtreePoint;

#[cfg(test)]
mod quadtree_tests {
use rust_comp_geo::{geometry::types::XY, quadtree::{quadtree::QuadtreeProps, types::{NodeGeometry, Quadrant}}};

use super::*;

fn setup() -> Quadtree<QuadtreePoint<String>, String> {
  let props: QuadtreeProps = QuadtreeProps {
    bounds: NodeGeometry {
      x_min: 0.0,
      x_max: 100.0,
      y_min: 0.0,
      y_max: 100.0,
    },
    max_objects: 2,
    max_levels: 4,
  };

  let quadtree: Quadtree<QuadtreePoint<String>, String> = Quadtree::new(props, 0);
  quadtree
}

  #[test]
  fn test_insert_object_to_quadtree() {
    let mut quadtree = setup();

    let object: QuadtreePoint<String> = QuadtreePoint::new(XY { x: 50.0, y: 50.0 }, "test".to_string());
    quadtree.insert(&object);

    assert_eq!(quadtree.objects.len(), 1);
    assert_eq!(quadtree.objects[0], object);

    quadtree.clear();
  }

  #[test]
  fn test_split_node_when_max_object() {
    let mut quadtree = setup();

    let object1: QuadtreePoint<String> = QuadtreePoint::new(XY { x: 25.0, y: 25.0 }, "object1".to_string());
    let object2: QuadtreePoint<String> = QuadtreePoint::new(XY { x: 25.0, y: 75.0 }, "object2".to_string());
    let object3: QuadtreePoint<String> = QuadtreePoint::new(XY { x: 75.0, y: 25.0 }, "object3".to_string());
    let object4: QuadtreePoint<String> = QuadtreePoint::new(XY { x: 75.0, y: 75.0 }, "object4".to_string());

    quadtree.insert(&object1);
    quadtree.insert(&object2);
    quadtree.insert(&object3);
    quadtree.insert(&object4);

    assert_eq!(quadtree.nodes.len(), 4);
    assert_eq!(quadtree.objects.len(), 0);
    assert!(quadtree.nodes[Quadrant::TopRight as usize].objects.contains(&object4));
    assert!(quadtree.nodes[Quadrant::TopLeft as usize].objects.contains(&object2));
    assert!(quadtree.nodes[Quadrant::BottomLeft as usize].objects.contains(&object1));
    assert!(quadtree.nodes[Quadrant::BottomRight as usize].objects.contains(&object3));

  }
}