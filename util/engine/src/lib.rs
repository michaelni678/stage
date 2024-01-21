#![allow(clippy::redundant_field_names)]
#![allow(clippy::too_many_arguments)]

mod app;
mod ecs;
mod error;
mod gfx;
mod math;
mod misc;
mod scene;
mod cmd;
mod ctx;

/* Exports. */
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
  texture::{TextureInfo, Textures, Texture},
  vertex::Vertex,
  color::Color,
};
pub use math::{Matrix4, Point, Scale};
pub use misc::{flag::Flag, hash::TypeIdHasher};
pub use scene::{Scene, Scenes, SceneError};
pub use app::{handlers::{App, AppSetupHandler, AppEventHandler, AppWindowEventHandler}, error::AppError};
pub use cmd::{command::{Command, commands::LoadScene}, queue::CommandQueue};
pub use ctx::Context;

use winit::event_loop::EventLoopWindowTarget;
/* Re-exports. */
pub use winit::window::WindowBuilder;
pub type ELWT = EventLoopWindowTarget<()>;
pub type Display = glium::Display<glium::glutin::surface::WindowSurface>;
