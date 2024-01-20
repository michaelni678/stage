use thiserror::Error;
use glium::{vertex, index, ProgramCreationError, SwapBuffersError, DrawError, texture::TextureCreationError};
use image::ImageError;

#[derive(Error, Debug)]
pub enum GfxError {
  #[error("Texture {0} was not found")]
  TextureNotFound(String),
  #[error("Sampler {0} was not found")]
  SamplerNotFound(u16),
  #[error("{0}")]
  VertexBufferCreation(#[from] vertex::BufferCreationError),
  #[error("{0}")]
  IndexBufferCreation(#[from] index::BufferCreationError),
  #[error("{0}")]
  ProgramCreation(#[from] ProgramCreationError),
  #[error("{0}")]
  TextureCreation(#[from] TextureCreationError),
  #[error("Failed to slice buffer")]
  BufferSlice,
  #[error("{0}")]
  SwapBuffers(#[from] SwapBuffersError),
  #[error("{0}")]
  Image(#[from] ImageError),
  #[error("{0}")]
  Draw(#[from] DrawError),
  #[error("No active camera to render with")]
  NoActiveCamera,
}