use thiserror::Error;

/// ECS-related errors.
#[derive(Error, Debug)]
pub enum EcsError {
  #[error("Entity was not found")]
  EntityNotFound,
  #[error("Unsatisfied inspect")]
  UnsatisfiedInspect,
}
