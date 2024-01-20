use crate::{
  Camera, Display, EngineError, GfxError, Pipeline, PipelineAttributes, Programs, Renderable,
  Textures, Transform, World,
};
use rustc_hash::FxHashMap;

/// Renders to the display.
pub struct Renderer {
  /// The GL context and facade.
  display: Display,
  /// The pipelines managed by the renderer.
  pipelines: FxHashMap<PipelineAttributes, Pipeline>,
  /// The program manager.
  programs: Programs,
  /// The texture manager.
  textures: Textures,
}

impl Renderer {
  /// Create a new renderer.
  pub fn new(display: Display) -> Result<Self, GfxError> {
    let programs = Programs::new(&display)?;
    let textures = Textures::new(&display)?;
    Ok(Self {
      display: display,
      pipelines: FxHashMap::default(),
      programs: programs,
      textures: textures,
    })
  }
  /// Execute the renderer.
  pub fn execute(&mut self, world: &mut World) -> Result<(), EngineError> {
    // Get a frame.
    let mut frame = self.display.draw();
    // Catch the execution results. This is done because the frame MUST be
    // destroyed, even if the execution of the renderer fails.
    let result = (|| {
      // Get the projection matrix.
      let projection = {
        // Get the active camera and inspect.
        let active_camera = world.actives.camera()?;
        let (transform, camera) = world.standard_inspect::<(&Transform, &Camera)>(active_camera)?;
        let fbd = self.display.get_framebuffer_dimensions();
        camera.projection(fbd, transform.position.into())
      };
      // Query the renderables.
      let query = world.standard_query::<(&Transform, &mut Renderable)>();
      for (_, (transform, renderable)) in query {
        // Get the texture information of the renderable.
        let texture_info = self.textures.get_texture_info(&renderable.texture)?;
        // Determine the pipeline attributes required to render the renderable.
        let pipeline_attrs = PipelineAttributes {
          index_pattern: renderable.mesh.indices(),
          sampler_id: texture_info.sampler_id,
        };
        // Get the pipeline, or create it if necessary.
        let pipeline = if let Some(pipeline) = self.pipelines.get_mut(&pipeline_attrs) {
          pipeline
        } else {
          let pipeline = Pipeline::new(&self.display, &pipeline_attrs, None)?;
          self.pipelines.entry(pipeline_attrs).or_insert(pipeline)
        };
        // Write to the pipeline.
        pipeline.write(
          &mut frame,
          &self.programs,
          &self.textures,
          &projection,
          transform.position,
          transform.scale,
          renderable.color,
          texture_info,
          &renderable.mesh,
        )?;
      }
      // Loop through the pipelines, flushing them.
      for pipeline in self.pipelines.values_mut() {
        pipeline.flush(&mut frame, &self.programs, &self.textures, projection)?;
      }
      Ok(())
    })();
    // Finish the frame.
    frame.finish().map_err(GfxError::from)?;
    // Return the result of the execution.
    result
  }
}
