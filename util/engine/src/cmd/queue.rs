use crate::{Command, EngineError, Context, Scenes};
use std::collections::VecDeque;

/// A queue of commands.
#[derive(Default)]
pub struct CommandQueue {
  /// The queue of commands.
  queue: VecDeque<Box<dyn Command>>,
}

impl CommandQueue {
  /// Create a new command queue.
  pub fn new() -> Self {
    Self::default()
  }
  /// Push a command to the queue.
  pub fn enqueue(&mut self, command: impl Command + 'static) {
    self.queue.push_back(Box::new(command));
  }
  /// Execute the commands in the queue.
  pub fn execute(&mut self, scenes: &mut Scenes, context: &mut Context) -> Result<(), EngineError> {
    while let Some(command) = self.queue.pop_front() {
      command.execute(self, scenes, context)?;
    }
    Ok(())
  }
}