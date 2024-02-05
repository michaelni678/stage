use crate::{Display, EngineError, Renderer, Simulator, World};

/// Holds a majority of the application's data.
pub struct Context {
  /// The renderer.
  pub renderer: Renderer,
  /// The world.
  pub world: World,
  /// The simulator.
  pub simulator: Simulator,
}

impl Context {
  /// Create a new context.
  pub fn new(display: Display) -> Result<Self, EngineError> {
    Ok(Self {
      renderer: Renderer::new(display)?,
      world: World::new(),
      simulator: Simulator::new(),
    })
  }
}
