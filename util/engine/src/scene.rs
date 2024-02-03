use crate::{CollisionEvent, CommandQueue, Context, EngineError, TypeIdHasher};
use std::{any::TypeId, collections::HashMap, hash::BuildHasherDefault};
use thiserror::Error;

/// Defines a scene.
pub trait Scene: 'static {
  /// Invoked when the scene is loaded.
  fn load(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError>;
  /// Invoked every frame.
  fn frame(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError>;
  /// Invoked at the end of the frame.
  fn postframe(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
    collision_events: Vec<CollisionEvent>,
  ) -> Result<(), EngineError>;
  /// Invoked when the scene is unloaded.
  fn unload(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError>;
}

/// Manages scenes.
pub struct Scenes {
  /// The currently loaded scene.
  loaded: TypeId,
  /// The registered scenes.
  scenes: HashMap<TypeId, Box<dyn Scene>, BuildHasherDefault<TypeIdHasher>>,
}

impl Default for Scenes {
  fn default() -> Self {
    // A dummy type id.
    let dummy_loaded = TypeId::of::<()>();
    // Return the scene manager.
    Self {
      loaded: dummy_loaded,
      scenes: HashMap::default(),
    }
  }
}

impl Scenes {
  /// Create a new scene manager.
  pub fn new() -> Self {
    Self::default()
  }
  /// Register a scene.
  pub fn register<S: Scene>(mut self, scene: S) -> Self {
    let tid = TypeId::of::<S>();
    self.scenes.insert(tid, Box::new(scene));
    self
  }
  /// Get the loaded scene.
  pub fn loaded(&mut self) -> Result<&mut dyn Scene, SceneError> {
    self
      .scenes
      .get_mut(&self.loaded)
      .map(|loaded| loaded.as_mut())
      .ok_or(SceneError::SceneNotFound)
  }
  /// Load a scene, unloading the previous scene.
  pub fn load<S: Scene>(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError> {
    // Get the previous scene, if there is one.
    if let Ok(prev_scene) = self.loaded() {
      // Unload the previous scene.
      prev_scene.unload(command_queue, context)?;
    }
    // Get the new scene.
    let tid = TypeId::of::<S>();
    let new_scene = self.scenes.get_mut(&tid).ok_or(SceneError::SceneNotFound)?;
    // Load the new scene.
    new_scene.load(command_queue, context)?;
    // Set the `loaded` field to the new scene's type id.
    self.loaded = tid;
    Ok(())
  }
}

/// Scene-related errors.
#[derive(Error, Debug)]
pub enum SceneError {
  #[error("Scene not found")]
  SceneNotFound,
}
