use thiserror::Error;
use winit::error::EventLoopError;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("{0}")]
  EventLoop(#[from] EventLoopError),
}