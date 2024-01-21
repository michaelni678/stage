#![allow(clippy::redundant_field_names)]
#![allow(clippy::too_many_arguments)]

mod ecs;
mod error;
mod gfx;
mod math;
mod misc;

pub use ecs::{
  actives::Actives,
  component::{
    components::{Camera, Renderable, Transform},
    Component,
  },
  entity::Entity,
  error::EcsError,
  world::World,
};
pub use error::EngineError;
pub use gfx::{
  error::GfxError,
  mesh::Mesh,
  pipeline::{Pipeline, PipelineAttributes},
  program::Programs,
  renderer::Renderer,
  texture::{TextureInfo, Textures, TextureKind},
  vertex::Vertex,
  color::Color,
};
pub use math::{Matrix4, Point, Scale};
pub use misc::flag::Flag;

pub type Display = glium::Display<glium::glutin::surface::WindowSurface>;
