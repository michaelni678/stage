use crate::{EngineError, CommandQueue, Context, Scenes};

/// Defines a command.
pub trait Command {
  /// Execute the command.
  fn execute(
    self: Box<Self>, 
    command_queue: &mut CommandQueue, 
    scenes: &mut Scenes,
    context: &mut Context,
  ) -> Result<(), EngineError>;
}

/// Command variants.
pub mod commands {
  use crate::{Command, Scene, EngineError, CommandQueue, Context, Scenes};

  /// Load a scene.
  #[ghost::phantom]
  pub struct LoadScene<S: Scene>;

  impl<S: Scene> Command for LoadScene<S> {
    fn execute(
      self: Box<Self>, 
      command_queue: &mut CommandQueue, 
      scenes: &mut Scenes,
      context: &mut Context,
    ) -> Result<(), EngineError> {
      scenes.load::<S>(command_queue, context)?;
      Ok(())
    }
  }
}