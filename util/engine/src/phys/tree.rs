use rstar::{RTree, RTreeObject};
use rustc_hash::FxHashMap;
use crate::{Point, Size, AABB};

/// A tree of colliders.
pub struct CollisionTree {
  inner: RTree<TreeObject>,
  environment_colliders: FxHashMap<u64, TreeObject>,
  next_id: u64,
}

impl Default for CollisionTree {
  fn default() -> Self {
    Self {
      inner: RTree::new(),
      environment_colliders: FxHashMap::default(),
      next_id: 0,
    }
  }
}

impl CollisionTree {
  /// Create a new collision tree.
  pub fn new() -> Self {
    Self::default()
  }
  /// Generate a new id.
  pub fn generate_id(&mut self) -> u64 {
    let id = self.next_id;
    self.next_id += 1;
    id
  }
  /// Add an environment collider to the tree.
  pub fn add_environment_collider(&mut self, position: Point, size: Size) -> u64 {
    // Generate an id for the environment collider.
    let id = self.generate_id();
    // Create the tree object.
    let obj = TreeObject {
      position: position,
      size: size,
      source: TreeObjectSource::Environment,
    };
    // Add to the tree and `environment_colliders`.
    self.environment_colliders.insert(id, obj);
    self.inner.insert(obj);
    id
  }
}

/// An object that is insertable into the collision tree.
#[derive(Copy, Clone)]
pub struct TreeObject {
  position: Point,
  size: Size,
  source: TreeObjectSource,
}

impl RTreeObject for TreeObject {
  type Envelope = AABB;
  fn envelope(&self) -> Self::Envelope {
    AABB::from_corners(self.position, self.position + self.size)
  }
}

/// Tree object source.
#[derive(Copy, Clone)]
pub enum TreeObjectSource {
  Environment,
}