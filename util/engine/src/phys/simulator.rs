use crate::{
  dynrect_vs_rect, Collider, CollisionEvent, CollisionTree, Point, Renderer, RigidBody, Size,
  Transform, TreeObjectSource, World,
};

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
    self
      .tree
      .add_collider(position.into(), size.into(), TreeObjectSource::Environment)
  }
  /// Execute the simulator.
  pub fn execute(
    &mut self,
    world: &mut World,
    renderer: &mut Renderer,
    timestep: f32,
  ) -> Vec<CollisionEvent> {
    // Collision events.
    // At the moment, this is just collision events between entities and colliders in the tree.
    let mut collision_events = Vec::with_capacity(0);
    // Query.
    let query = world.standard_query::<(&mut Transform, &mut RigidBody, &Collider)>();
    // Colliders inserted temporarily.
    let mut collider_insertions = Vec::new();
    for (entity, (transform, rigid_body, collider)) in query {
      // Broad phase against tree.
      let broad_phase = self.tree.broad_phase(
        transform.position + collider.offset,
        collider.size,
        rigid_body.velocity,
        timestep,
      );
      // Create the source.
      let source = TreeObjectSource::Entity { handle: entity };
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
          rigid_body.velocity += collision
            .contact_normal
            .component_mul(&rigid_body.velocity.abs())
            * (1.0 - collision.contact_time);
          // Create the collision event and push it to `collision_events`.
          let collision_event = CollisionEvent {
            source1: source,
            source2: static_object.source,
          };
          collision_events.push(collision_event);
        }
      }
      // Narrow phase against other dynamic colliders.
      // !TODO: Implement this.

      // Set the new position with the corrected velocity.
      transform.position = transform.position + rigid_body.velocity * timestep;
      // Add the entity to the collision tree temporarily.
      let id = self
        .tree
        .add_collider(transform.position + collider.offset, collider.size, source);
      collider_insertions.push(id);
      // Apply the acceleration to the velocity.
      rigid_body.velocity += rigid_body.acceleration * timestep;
    }
    // If enabled, draw the corners of the colliders in the tree.
    #[cfg(feature = "show_hitboxes")]
    {
      self.tree.draw_collider_corners(renderer);
    }
    // This is just to fix linting.
    // The renderer is only used for the "show_hitboxes" feature.
    let _ = renderer;
    // Remove the insertions.
    for id in collider_insertions {
      self.tree.remove_collider(id);
    }
    // Return the collision events.
    collision_events
  }
}
