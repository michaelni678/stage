use beyond_engine::{
  App, AppEventHandler, AppSetupHandler, AppWindowEventHandler, Camera, Collider, Color,
  CommandQueue, Context, EngineError, LoadScene, Mesh, RenderRequest, Renderable, RigidBody, Scene,
  Scenes, Texture, Transform, WindowBuilder, ELWT,
};

/// An example application.
pub struct ExampleApp;

impl App for ExampleApp {}

impl AppSetupHandler for ExampleApp {
  fn window(window_builder: WindowBuilder) -> WindowBuilder {
    window_builder.with_title("Beyond Engine Physics Example")
  }
  fn scenes(scenes: Scenes) -> Scenes {
    scenes.register(ExampleScene)
  }
}

impl AppEventHandler for ExampleApp {
  fn init(
    &mut self,
    command_queue: &mut CommandQueue,
    _context: &mut Context,
  ) -> Result<(), EngineError> {
    // Load the example scene.
    command_queue.enqueue(LoadScene::<ExampleScene>);
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
    let entity = context.world.spawn_entity((
      Transform::new([0.0, 0.0], [128.0, 128.0]),
      Renderable::new(Color::rgb(1.0, 0.0, 0.0), Texture::none(), Mesh::square()),
      Camera::new([64.0, 64.0]),
      RigidBody::new([0.0, 15.0]),
      Collider::new([0.0, 0.0], [128.0, 128.0]),
    ));
    context.world.actives.set_camera(entity);
    // Add an environment collider.
    context
      .simulator
      .add_environment_collider([0.0, 256.0], [128.0, 128.0]);
    Ok(())
  }
  fn frame(
    &mut self,
    _command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError> {
    context.renderer.add_render_request(RenderRequest::square(
      [0.0, 256.0],
      [128.0, 128.0],
      Color::green(),
      Texture::none(),
    ));
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

fn main() -> Result<(), EngineError> {
  Box::new(ExampleApp).run()
}
