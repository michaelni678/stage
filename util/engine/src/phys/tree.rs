use rstar::{RTree, RTreeObject};
use crate::{Point, Size, AABB};

pub struct CollisionTree {
  inner: RTree<TreeObject>,
}

impl Default for CollisionTree {
  fn default() -> Self {
    Self {
      inner: RTree::new(),
    }
  }
}

impl CollisionTree {
  /// Create a new collision tree.
  pub fn new() -> Self {
    Self::default()
  }
}

pub struct TreeObject {
  position: Point,
  size: Size,
}

impl RTreeObject for TreeObject {
  type Envelope = AABB;
  fn envelope(&self) -> Self::Envelope {
    AABB::from_corners(self.position, self.position + self.size)
  }
}