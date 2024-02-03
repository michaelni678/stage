#![allow(clippy::redundant_field_names)]
#![allow(clippy::too_many_arguments)]

mod app;
mod cmd;
mod ctx;
mod ecs;
mod error;
mod gfx;
mod math;
mod misc;
mod phys;
mod scene;

/* Exports. */
pub use app::{
  error::AppError,
  handlers::{App, AppEventHandler, AppSetupHandler, AppWindowEventHandler},
};
pub use cmd::{
  command::{commands::LoadScene, Command},
  queue::CommandQueue,
};
pub use ctx::Context;
pub use ecs::{
  actives::Actives,
  component::{
    components::{Camera, Collider, Renderable, RigidBody, Transform},
    Component,
  },
  entity::Entity,
  error::EcsError,
  world::World,
};
pub use error::EngineError;
pub use gfx::{
  color::Color,
  error::GfxError,
  mesh::Mesh,
  pipeline::{Pipeline, PipelineAttributes},
  program::Programs,
  renderer::Renderer,
  request::RenderRequest,
  texture::{Texture, TextureInfo, Textures},
  vertex::Vertex,
};
pub use math::{Matrix4, Point, Ray, Scale, Size, Vector, AABB};
pub use misc::{flag::Flag, hash::TypeIdHasher};
pub use phys::{
  query::*,
  simulator::Simulator,
  tree::{CollisionEvent, CollisionTree, TreeObject, TreeObjectSource},
};
pub use scene::{Scene, SceneError, Scenes};

/* Re-exports. */
pub use winit::window::WindowBuilder;
pub type ELWT = winit::event_loop::EventLoopWindowTarget<()>;
pub type Display = glium::Display<glium::glutin::surface::WindowSurface>;
