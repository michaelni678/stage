use crate::{Renderer, World, Display, EngineError};

/// Holds a majority of the application's data.
pub struct Context {
  /// The renderer.
  pub renderer: Renderer,
  /// The world.
  pub world: World,
}

impl Context {
  /// Create a new context.
  pub fn new(display: Display) -> Result<Self, EngineError> {
    Ok(Self {
      renderer: Renderer::new(display)?,
      world: World::new(),
    })
  }
}