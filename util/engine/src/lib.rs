mod gfx;
mod ecs;
mod math;
mod misc;
mod error;

pub use gfx::{renderer::Renderer, pipeline::{Pipeline, PipelineAttributes}, error::GfxError, mesh::Mesh, vertex::Vertex, texture::{Textures, TextureInfo}, program::Programs};
pub use ecs::{world::World, error::EcsError, entity::Entity, component::{Component, components::{Transform, Renderable, Camera}}, actives::Actives};
pub use math::{Point, Scale, Matrix4};
pub use misc::flag::Flag;
pub use error::EngineError;

pub type Display = glium::Display<glium::glutin::surface::WindowSurface>;