use rstar::{iterators::LocateInEnvelopeIntersecting, Envelope, RTree, RTreeObject};
use rustc_hash::FxHashMap;
use crate::{Entity, Point, Size, Vector, AABB};

/// A tree of colliders.
pub struct CollisionTree {
  inner: RTree<TreeObject>,
  colliders: FxHashMap<u64, TreeObject>,
  next_id: u64,
}

impl Default for CollisionTree {
  fn default() -> Self {
    Self {
      inner: RTree::new(),
      colliders: FxHashMap::default(),
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
  /// Add a collider to the tree.
  pub fn add_collider(&mut self, position: Point, size: Size, source: TreeObjectSource) -> u64 {
    // Generate an id for the collider.
    let id = self.generate_id();
    // Create the tree object.
    let obj = TreeObject {
      position: position,
      size: size,
      source: source,
    };
    // Add to the tree and `colliders`.
    self.colliders.insert(id, obj);
    self.inner.insert(obj);
    id
  }
  /// Broad phase.
  pub fn broad_phase(
    &self, 
    position: Point, 
    size: Size, 
    velocity: Vector, 
    timestep: f32,
  ) -> LocateInEnvelopeIntersecting<TreeObject> {
    // Calculate the start and end AABBs. Merge them to create the
    // broad phase search area.
    let start = AABB::from_corners(position, position + size);
    let end = {
      let min_end = position + velocity * timestep;
      AABB::from_corners(min_end, min_end + size)
    };
    let search = start.merged(&end);
    self.inner.locate_in_envelope_intersecting(&search)
  }
}

/// An object that is insertable into the collision tree.
#[derive(Copy, Clone)]
pub struct TreeObject {
  pub position: Point,
  pub size: Size,
  pub source: TreeObjectSource,
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
  Entity {
    handle: Entity,
  }
}