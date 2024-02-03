use std::mem;

use crate::{Point, Ray, Size, Vector};

/// Collision information.
#[derive(Debug)]
pub struct Collision {
  pub contact_time: f32,
  pub contact_point: Point,
  pub contact_normal: Vector,
}

/// Query if a ray intersects a rect.
pub fn ray_vs_rect(ray: Ray, position: Point, size: Size) -> Option<Collision> {
  // Inverse the ray.
  let inverse_direction = ray.direction.map(|i| i.recip());
  // Calculate the times.
  let mut t_near = (position - ray.origin) * inverse_direction;
  let mut t_far = (position + size - ray.origin) * inverse_direction;
  // Reject if NaN.
  if t_near.has_nan() || t_far.has_nan() {
    None?
  }
  // Sort distances.
  if t_near.x > t_far.x {
    mem::swap(&mut t_near.x, &mut t_far.x);
  }
  if t_near.y > t_far.y {
    mem::swap(&mut t_near.y, &mut t_far.y);
  }
  // Early rejection.
  if t_near.x > t_far.y || t_near.y > t_far.x {
    None?
  }
  // Contact time and far contact time.
  let contact_time = t_near.x.max(t_near.y);
  let far_contact_time = t_far.x.min(t_far.y);
  // Reject if ray direction is pointing away from the object.
  if far_contact_time < 0.0 {
    None?
  }
  // Contact point.
  let contact_point = ray.origin + contact_time * ray.direction;
  // Contact normal.
  let contact_normal = if t_near.x > t_near.y {
    Vector::new(-inverse_direction.x.signum(), 0.0)
  } else if t_near.x < t_near.y {
    Vector::new(0.0, -inverse_direction.y.signum())
  } else {
    Vector::zeros()
  };
  Some(Collision {
    contact_time: contact_time,
    contact_point: contact_point,
    contact_normal: contact_normal,
  })
}

/// Query if a moving rect intersects a stationary rect.
pub fn dynrect_vs_rect(
  position1: Point,
  size1: Size,
  velocity: Vector,
  position2: Point,
  size2: Size,
  timestep: f32,
) -> Option<Collision> {
  let ray = Ray::new(position1 + size1 / 2.0, velocity * timestep);
  let result = ray_vs_rect(ray, position2 - (size1 / 2.0), size2 + size1);
  match result {
    Some(ref collision) if collision.contact_time >= 0.0 && collision.contact_time < 1.0 => result,
    _ => None,
  }
}
