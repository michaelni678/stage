use crate::{AppError, EcsError, GfxError, SceneError};
use thiserror::Error;

/// Engine errors.
#[derive(Error, Debug)]
pub enum EngineError {
  #[error("{0}")]
  Gfx(#[from] GfxError),
  #[error("{0}")]
  Ecs(#[from] EcsError),
  #[error("{0}")]
  App(#[from] AppError),
  #[error("{0}")]
  Scene(#[from] SceneError),
}
