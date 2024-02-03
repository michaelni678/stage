use crate::{AppError, CommandQueue, Context, EngineError, Scenes, WindowBuilder, ELWT};
use glium::backend::glutin::SimpleWindowBuilder;
use winit::{
  event::{Event, StartCause, WindowEvent},
  event_loop::EventLoop,
};

/// Defines an application.
pub trait App: AppSetupHandler + AppEventHandler + AppWindowEventHandler {
  fn run(mut self: Box<Self>) -> Result<(), EngineError> {
    // Create the event loop.
    let event_loop = EventLoop::new().map_err(AppError::from)?;
    // Create the window and display.
    let (window, display) = {
      let window_builder = WindowBuilder::new();
      SimpleWindowBuilder::new()
        .set_window_builder(Self::window(window_builder))
        .build(&event_loop)
    };
    // Create the command queue.
    let mut command_queue = CommandQueue::new();
    // Create the scene manager.
    let mut scenes = {
      let scenes = Scenes::new();
      Self::scenes(scenes)
    };
    // Create the context.
    let mut context = Context::new(display)?;
    // Run the event loop.
    event_loop
      .run(move |event, elwt| {
        if let Err(error) = (|| {
          match event {
            // Init event.
            Event::NewEvents(StartCause::Init) => {
              // Invoke the application init event.
              self.init(&mut command_queue, &mut context)?;
            },
            // Exit event.
            Event::LoopExiting => self.exit(&mut command_queue, &mut context)?,
            // Window events.
            Event::WindowEvent { window_id, event } => {
              // Assert that the id of the window that emitted the event
              // and the window that the user sees are the same.
              assert_eq!(window.id(), window_id);
              // Match the window event.
              match event {
                // Close request event.
                WindowEvent::CloseRequested => {
                  self.close_request(elwt, &mut command_queue, &mut context)?
                },
                // Redraw request.
                WindowEvent::RedrawRequested => {
                  // Execute the command queue.
                  command_queue.execute(&mut scenes, &mut context)?;
                  // Get the scene.
                  let scene = scenes.loaded()?;
                  // Execute the scene frame.
                  scene.frame(&mut command_queue, &mut context)?;
                  // Execute the simulator.
                  let collision_events = context.simulator.execute(
                    &mut context.world,
                    &mut context.renderer,
                    1.0 / 30.0,
                  );
                  // Execute the renderer.
                  context.renderer.execute(&mut context.world)?;
                  // Execute the scene postframe.
                  scene.postframe(&mut command_queue, &mut context, collision_events)?;
                },
                // Ignore other window events.
                _ => (),
              }
            },
            // Request a redraw.
            Event::AboutToWait => window.request_redraw(),
            // Ignore other events.
            _ => (),
          }
          Ok::<(), EngineError>(())
        })() {
          eprintln!("{}", error);
        }
      })
      .map_err(AppError::from)?;
    Ok(())
  }
}

/// Handles application setup.
pub trait AppSetupHandler {
  /// Specifies how the window is set up.
  fn window(window_builder: WindowBuilder) -> WindowBuilder;
  /// Register scenes.
  fn scenes(scenes: Scenes) -> Scenes;
}

/// Handles application events.
pub trait AppEventHandler {
  /// Handle the application init event, sent immediately after run is called.
  fn init(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError>;
  /// Handle the application exit event, sent when event polling is finished.
  fn exit(
    &mut self,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError>;
}

/// Handles application window events.
pub trait AppWindowEventHandler {
  /// Handle the close request event.
  fn close_request(
    &mut self,
    elwt: &ELWT,
    command_queue: &mut CommandQueue,
    context: &mut Context,
  ) -> Result<(), EngineError>;
}
