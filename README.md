# Stage
Stage is a simple 2-D game engine.
It's more of a hobby project and lacks many features an established game engine would have.

## Features
* App framework:
  - `App` trait for application events (init, exit, etc.), `Scene` trait for scene events (load, frame, unload, etc.).
* Basic rendering:
  - Entity-based rendering; Entities with the `Renderable` component are rendered automatically.
  - Render requests; Per-frame rendering requests useful for debugging or drawing non-entity meshes.
  - Uses *glium* / *OpenGL* and *GLSL*.
  - **TODO:** Immutable buffers for static scenes. At the moment, the only way to draw a scene (without having the scene as an entity) is to use a render request, which writes to the buffers each frame. This is unnecessary if the scene is static. Allow the developer to declare a pipeline with immutable buffers for drawing a static mesh.
* Basic physics:
  - Entity-based physics; Entities with the `RigidBody` component are moved automatically. Entities with the `Collider` component will collide with other colliders.
  - Environment colliders; Static non-entity colliders can be added to the physics simulator.
  - Uses *rstar*'s R-Tree implementation for broad-phase collision detection. Uses AABB CCD during narrow-phase.
  - **TODO:** The timestep is currently hardcoded and thus physics is simulated differently between different machines. The game loop should have a fixed timestep for physics simulation which can be set via commands. The best approach would probably be to create a `Timer` struct, which should also count the frames, cache the app start system time, etc.
 
## Usage
Stage isn't on crates.io, so you'll have to link the repository directly in Cargo.toml.
This is a hobby project that lacks many major features, so I don't recommend using it over something like Bevy, Unity, UE, etc.
