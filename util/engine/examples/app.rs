use beyond_engine::{
  App, AppEventHandler, AppSetupHandler, AppWindowEventHandler, Camera, Color, CommandQueue,
  Context, EngineError, LoadScene, Mesh, Point, Renderable, Scene, Scenes, Texture, Transform,
  WindowBuilder, ELWT,
};

/// An example application.
pub struct ExampleApp;

impl App for ExampleApp {}

impl AppSetupHandler for ExampleApp {
  fn window(window_builder: WindowBuilder) -> WindowBuilder {
    window_builder.with_title("Beyond Engine Application Example")
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
      Renderable::new(
        Color::rgb(1.0, 0.0, 0.0),
        Texture::none(),
        Mesh::new(
          vec![
            Point::new(-0.5, -0.5),
            Point::new(0.5, -0.5),
            Point::new(0.5, 0.5),
            Point::new(-0.5, 0.5),
          ],
          vec![0, 2, 1, 0, 3, 2],
        ),
      ),
      Camera::new([0.0, 0.0]),
    ));
    context.world.actives.set_camera(entity);
    Ok(())
  }
  fn frame(
    &mut self,
    _command_queue: &mut CommandQueue,
    _context: &mut Context,
  ) -> Result<(), EngineError> {
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
