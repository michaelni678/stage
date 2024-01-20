use thiserror::Error;
use crate::{GfxError, EcsError};

/// Engine errors.
#[derive(Error, Debug)]
pub enum EngineError {
  #[error("{0}")]
  Gfx(#[from] GfxError),
  #[error("{0}")]
  Ecs(#[from] EcsError),
}