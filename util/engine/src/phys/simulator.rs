use crate::CollisionTree;

/// Simulates physics.
#[derive(Default)]
pub struct Simulator {
  tree: CollisionTree,
}

impl Simulator {
  /// Create a new simulator.
  pub fn new() -> Self {
    Self::default()
  }
}