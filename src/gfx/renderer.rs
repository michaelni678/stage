use crate::{
  Camera, Display, EngineError, GfxError, Pipeline, PipelineAttributes, Programs, Renderable,
  Textures, Transform, World,
};
use glium::Surface;
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
  /// The render requests.
  render_requests: Vec<(Transform, Renderable)>,
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
      render_requests: Vec::new(),
    })
  }
  /// Add a new sampler.
  /// Returns it's id.
  pub fn add_sampler(
    &mut self,
    bytes: impl AsRef<[u8]>,
    info: impl IntoIterator<Item = (impl ToString, Vec<[f32; 2]>)>,
  ) -> Result<u16, GfxError> {
    self.textures.add_sampler(&self.display, bytes, info)
  }
  /// Add a new render request.
  pub fn add_render_request(&mut self, request: (Transform, Renderable)) {
    self.render_requests.push(request);
  }
  /// Execute the renderer.
  pub fn execute(&mut self, world: &mut World) -> Result<(), EngineError> {
    // Get a frame and clear it.
    let mut frame = self.display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    // Catch the execution results. This is done because the frame MUST be
    // destroyed, even if the execution of the renderer fails.
    let result = (|| {
      // Get the projection matrix.
      let projection = {
        // Get the active camera and inspect.
        let active_camera = world.actives.camera()?;
        let (transform, camera) = world.standard_inspect::<(&Transform, &Camera)>(active_camera)?;
        let fbd = self.display.get_framebuffer_dimensions();
        camera.projection(fbd, transform.position)
      };
      // Query the renderables.
      let query = world
        .standard_query::<(&Transform, &mut Renderable)>()
        .into_iter()
        .map(|(_, data)| data);
      let requests = self.render_requests.iter_mut().map(|(t, r)| (&*t, r));
      let chain = query.into_iter().chain(requests);
      for (transform, renderable) in chain {
        // Get the texture information of the renderable.
        let texture_info = self.textures.get_texture_info(renderable.texture.get())?;
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
          renderable.color.into(),
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
    // Clear the render requests.
    self.render_requests.clear();
    // Finish the frame.
    frame.finish().map_err(GfxError::from)?;
    // Return the result of the execution.
    result
  }
}
