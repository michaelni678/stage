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
* Basic physics:
  - Entity-based physics; Entities with the `RigidBody` component are moved automatically. Entities with the `Collider` component will collide with other colliders.
  - Environment colliders; Static non-entity colliders can be added to the physics simulator.
  - Uses *rstar*'s R-Tree implementation for broad-phase collision detection. Uses AABB CCD during narrow-phase.
