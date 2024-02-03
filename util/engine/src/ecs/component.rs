/// Defines a component.
pub trait Component: Send + Sync {}

/// Component variants.
pub mod components {
  use crate::{Color, Component, Matrix4, Mesh, Point, Scale, Size, Texture, Vector};

  /// The position and scale of an entity.
  pub struct Transform {
    pub position: Point,
    pub scale: Scale,
  }

  impl Transform {
    /// Create a new transform component.
    pub fn new(position: impl Into<Point>, scale: impl Into<Scale>) -> Self {
      Self {
        position: position.into(),
        scale: scale.into(),
      }
    }
  }

  impl Component for Transform {}

  /// Allows an entity to be rendered.
  pub struct Renderable {
    pub color: Color,
    pub texture: Texture,
    pub mesh: Mesh,
  }

  impl Renderable {
    /// Create a new renderable component.
    pub fn new(color: Color, texture: Texture, mesh: Mesh) -> Self {
      Self {
        color: color,
        texture: texture,
        mesh: mesh,
      }
    }
  }

  impl Component for Renderable {}

  /// The camera component.
  pub struct Camera {
    pub offset: [f32; 2],
  }

  impl Camera {
    /// Create a new camera.
    pub fn new(offset: [f32; 2]) -> Self {
      Self { offset: offset }
    }
    /// Get the projection matrix.
    /// `fbd` is the frame buffer dimensions.
    /// `position` is the position of the entity that holds the camera.
    pub fn projection(&self, fbd: (u32, u32), position: Point) -> [[f32; 4]; 4] {
      let left = position.x - (fbd.0 / 2) as f32 + self.offset[0];
      let right = position.x + (fbd.0 / 2) as f32 + self.offset[0];
      let bottom = position.y + (fbd.1 / 2) as f32 + self.offset[1];
      let top = position.y - (fbd.1 / 2) as f32 + self.offset[1];
      Matrix4::new_orthographic(left, right, bottom, top, -1.0, 1.0).into()
    }
  }

  impl Component for Camera {}

  /// The rigid body component.
  pub struct RigidBody {
    /// The velocity of the rigid body.
    pub velocity: Vector,
    /// Acceleration of the rigid body.
    pub acceleration: Vector,
  }

  impl RigidBody {
    /// Create a new rigid body.
    pub fn new(velocity: impl Into<Vector>) -> Self {
      Self {
        velocity: velocity.into(),
        acceleration: Vector::new(0.0, 9.81),
      }
    }
  }

  impl Component for RigidBody {}

  /// The collider component.
  pub struct Collider {
    pub offset: Point,
    pub size: Size,
  }

  impl Collider {
    /// Create a new collider.
    pub fn new(offset: impl Into<Point>, size: impl Into<Size>) -> Self {
      Self {
        offset: offset.into(),
        size: size.into(),
      }
    }
  }

  impl Component for Collider {}
}
