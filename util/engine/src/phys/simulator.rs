use crate::{dynrect_vs_rect, ray_vs_rect, Collider, CollisionTree, Point, Ray, RigidBody, Size, Transform, TreeObjectSource, World};

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
    self.tree.add_collider(position.into(), size.into(), TreeObjectSource::Environment)
  }
  /// Execute the simulator.
  pub fn execute(&mut self, world: &mut World, timestep: f32) {
    let query = world.standard_query::<(&mut Transform, &mut RigidBody, &Collider)>();
    let mut collider_insertions = Vec::new();
    for (entity, (transform, rigid_body, collider)) in query {
      // Broad phase against tree.
      let broad_phase = self.tree.broad_phase(
        transform.position + collider.offset, 
        collider.size, 
        rigid_body.velocity, 
        timestep,
      );
      // Narrow phase against statics returned by the broad phase.
      for static_object in broad_phase {
        if let Some(collision) = dynrect_vs_rect(
          transform.position + collider.offset, 
          collider.size, 
          rigid_body.velocity, 
          static_object.position, 
          static_object.size, 
          timestep,
        ) {
          // Adjust the velocity.
          rigid_body.velocity = rigid_body.velocity + collision.contact_normal.component_mul(&rigid_body.velocity.abs()) * (1.0 - collision.contact_time);
        }
      }
      // Narrow phase against other dynamic colliders.

      // Set the new position with the corrected velocity.
      transform.position = transform.position + rigid_body.velocity * timestep;
      // Add the entity to the collision tree temporarily.
      let id = self.tree.add_collider(transform.position, collider.size, TreeObjectSource::Entity { handle: entity });
      collider_insertions.push(id);
    }
  }
}