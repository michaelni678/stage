use beyond_engine::{Camera, Mesh, Point, Renderable, Renderer, Transform, World};
use glium::backend::glutin::SimpleWindowBuilder;
use winit::{
  event::{Event, WindowEvent},
  event_loop::EventLoop,
};

fn main() {
  // Application setup.
  let event_loop = EventLoop::new().unwrap();
  let (window, display) = SimpleWindowBuilder::new()
    .with_title("Beyond Engine Graphics Example")
    .build(&event_loop);
  // Create the renderer.
  let mut renderer = Renderer::new(display).unwrap();
  // Create the world.
  let mut world = World::new();
  // Spawn an entity.
  let entity = world.spawn_entity((
    Transform::new([0.0, 0.0], [128.0, 128.0]),
    Renderable::new(
      Some([1.0, 0.0, 0.0, 1.0]),
      None,
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
  world.actives.set_camera(entity);
  // Run the event loop.
  event_loop
    .run(move |event, elwt| match event {
      Event::AboutToWait => window.request_redraw(),
      Event::WindowEvent { window_id, event } => {
        assert_eq!(window.id(), window_id);
        match event {
          WindowEvent::RedrawRequested => {
            renderer.execute(&mut world).unwrap();
          },
          WindowEvent::CloseRequested => elwt.exit(),
          _ => (),
        }
      },
      _ => (),
    })
    .unwrap();
}
