use beyond_engine::{Camera, Mesh, Point, Renderable, Renderer, Transform, World, include_wrt_manifest};
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
  // Add the samplers.
  renderer
    .add_sampler(
      include_wrt_manifest!("/examples/standalone.png"),
      [(
        "Standalone",
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
      )],
    )
    .unwrap();
  renderer
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
  // Spawn more entities.
  world.spawn_entity((
    Transform::new([-320.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      None,
      Some(String::from("Standalone")),
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
  ));
  world.spawn_entity((
    Transform::new([-192.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      None,
      Some(String::from("Atlas Full")),
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
  ));
  world.spawn_entity((
    Transform::new([-64.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      None,
      Some(String::from("Atlas 0")),
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
  ));
  world.spawn_entity((
    Transform::new([64.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      None,
      Some(String::from("Atlas 1")),
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
  ));
  world.spawn_entity((
    Transform::new([192.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      None,
      Some(String::from("Atlas 2")),
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
  ));
  world.spawn_entity((
    Transform::new([320.0, 128.0], [128.0, 128.0]),
    Renderable::new(
      None,
      Some(String::from("Atlas 3")),
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
  ));
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
