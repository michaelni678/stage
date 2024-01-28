use crate::{Color, Mesh, Point, Renderable, Scale, Texture, Transform};

/// Geometry primitives for render requests.
pub struct Geometry;

impl Geometry {
  /// A point.
  // !TODO
  // The point is currently actually a small square instead of a circle.
  // Change to a circle mesh.
  pub fn point(position: impl Into<Point>, color: Color) -> (Transform, Renderable) {
    Geometry::square(position, [8.0, 8.0], color, Texture::None)
  }
  /// A square.
  pub fn square(
    position: impl Into<Point>,
    scale: impl Into<Scale>,
    color: Color,
    texture: Texture,
  ) -> (Transform, Renderable) {
    let mesh = Mesh::new(
      vec![
        Point::new(-0.5, -0.5),
        Point::new(0.5, -0.5),
        Point::new(0.5, 0.5),
        Point::new(-0.5, 0.5),
      ],
      vec![0, 2, 1, 0, 3, 2],
    );
    (
      Transform::new(position, scale),
      Renderable::new(color, texture, mesh),
    )
  }
}
