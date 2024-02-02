use crate::{CollisionTree, Point, Size, World};

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
  /// Add an environment collider to the simulator.
  pub fn add_environment_collider(
    &mut self, 
    position: impl Into<Point>, 
    size: impl Into<Size>,
  ) -> u64 {
    self.tree.add_environment_collider(position.into(), size.into())
  }
  /// Execute the simulator.
  pub fn execute(&mut self, world: &mut World) {

  }
}