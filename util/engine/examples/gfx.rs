use beyond_engine::{
  include_wrt_manifest, App, AppEventHandler, AppSetupHandler, AppWindowEventHandler, Camera,
  Color, CommandQueue, Context, EngineError, LoadScene, Mesh, Point, RenderRequest, Renderable,
  Renderer, Scene, Scenes, Texture, Transform, WindowBuilder, World, ELWT,
};

/// An example application.
pub struct ExampleApp;

impl App for ExampleApp {}

impl AppSetupHandler for ExampleApp {
  fn window(window_builder: WindowBuilder) -> WindowBuilder {
    window_builder.with_title("Beyond Engine Graphics Example")
  }
  fn scenes(scenes: Scenes) -> Scenes {
    scenes.register(ExampleScene)
  }
}

impl AppEventHandler for ExampleApp {
  fn init(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError> {
    // Load the example scene.
    command_queue.enqueue(LoadScene::<ExampleScene>);
    // Add the samplers.
    context
      .renderer
      .add_sampler(
        include_wrt_manifest!("/examples/standalone.png"),
        [(
          "Standalone",
          vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        )],
      )
      .unwrap();
    context
      .renderer
      .add_sampler(
        include_wrt_manifest!("/examples/atlas.png"),
        [
          (
            "Atlas Full",
            vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
          ),
          (
            "Atlas 0",
            vec![[0.0, 0.0], [0.5, 0.0], [0.5, 0.5], [0.0, 0.5]],
          ),
          (
            "Atlas 1",
            vec![[0.0, 0.5], [0.5, 0.5], [0.5, 1.0], [0.0, 1.0]],
          ),
          (
            "Atlas 2",
            vec![[0.5, 0.0], [1.0, 0.0], [1.0, 0.5], [0.5, 0.5]],
          ),
          (
            "Atlas 3",
            vec![[0.5, 0.5], [1.0, 0.5], [1.0, 1.0], [0.5, 1.0]],
          ),
        ],
      )
      .unwrap();
    Ok(())
  }
  fn exit(
    &mut self,
    _command_queue: &mut CommandQueue,
    _context: &mut Context,
  ) -> Result<(), EngineError> {
    Ok(())
  }
}

impl AppWindowEventHandler for ExampleApp {
  fn close_request(
    &mut self,
    elwt: &ELWT,
    _command_queue: &mut CommandQueue,
    _context: &mut Context,
  ) -> Result<(), EngineError> {
    elwt.exit();
    Ok(())
  }
}

/// An example scene.
pub struct ExampleScene;

impl Scene for ExampleScene {
  fn load(
    &mut self,
    _command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError> {
    // Spawn the active entity.
    // Solid red renderable.
    let entity = context.world.spawn_entity((
      Transform::new([0.0, 0.0], [128.0, 128.0]),
      Renderable::new(
        Color::rgb(1.0, 0.0, 0.0),
        Texture::none(),
        Mesh::new(
          vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
          ],
          vec![0, 2, 1, 0, 3, 2],
        ),
      ),
      Camera::new([0.0, 0.0]),
    ));
    context.world.actives.set_camera(entity);
    // Spawn more entities.
    more_entities(&mut context.world);
    Ok(())
  }
  fn frame(
    &mut self,
    _command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError> {
    requests(&mut context.renderer);
    Ok(())
  }
  fn unload(
    &mut self,
    _command_queue: &mut CommandQueue,
    _context: &mut Context,
  ) -> Result<(), EngineError> {
    Ok(())
  }
}

/// Spawn more entities.
fn more_entities(world: &mut World) {
  // Standalone texture renderable.
  world.spawn_entity((
    Transform::new([-320.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      Color::none(),
      Texture::regular("Standalone"),
      Mesh::square(),
    ),
  ));
  // Full atlas texture renderable.
  world.spawn_entity((
    Transform::new([-192.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      Color::none(),
      Texture::regular("Atlas Full"),
      Mesh::square(),
    ),
  ));
  // Atlas subtexture renderable.
  world.spawn_entity((
    Transform::new([-64.0, 128.0], [128.0, 128.0]),
    Renderable::new(Color::none(), Texture::regular("Atlas 0"), Mesh::square()),
  ));
}

/// Per-frame render requests.
fn requests(renderer: &mut Renderer) {
  // Red-tinted atlas subtexture renderable.
  renderer.add_render_request((
    Transform::new([64.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      Color::rgba(1.0, 0.75, 0.75, 1.0),
      Texture::regular("Atlas 1"),
      Mesh::square(),
    ),
  ));
  // Semi-transparent atlas subtexture renderable.
  renderer.add_render_request((
    Transform::new([192.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      Color::alpha(0.1),
      Texture::regular("Atlas 2"),
      Mesh::square(),
    ),
  ));
  // Small atlas subtexture renderable.
  renderer.add_render_request((
    Transform::new([320.0, 128.0], [64.0, 64.0]),
    Renderable::new(Color::none(), Texture::regular("Atlas 3"), Mesh::square()),
  ));
  // A blue point.
  renderer.add_render_request(RenderRequest::point([0.0, 0.0], Color::blue()));
}

fn main() -> Result<(), EngineError> {
  Box::new(ExampleApp).run()
}
