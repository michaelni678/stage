use crate::{Flag, Point};

/// A mesh.
pub struct Mesh {
  /// The vertices of the mesh.
  vertices: Flag<Box<[Point]>>,
  /// The indices of the mesh. Must be recalculated if
  /// `vertices` is changed.
  indices: Box<[u32]>,
}

impl Mesh {
  /// Create a new mesh.
  pub fn new(vertices: Vec<Point>, indices: Vec<u32>) -> Self {
    let vertices = vertices.into_boxed_slice();
    Self {
      vertices: Flag::new_clean(vertices),
      indices: indices.into_boxed_slice(),
    }
  }
  /// Get the indices of the mesh.
  pub fn vertices(&self) -> &[Point] {
    self.vertices.get().as_ref()
  }
  /// Get the indices of the mesh.
  pub fn indices(&mut self) -> Box<[u32]> {
    if self.vertices.is_dirty() {
      todo!();
      self.vertices.clean();
    }
    self.indices.clone()
  }
}
