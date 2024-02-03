use crate::{Point, Ray, Size, Vector};

/// Collision information.
pub struct Collision {
  pub contact_time: f32,
  pub contact_point: Point,
  pub contact_normal: Vector,
}

/// Query if a ray intersects a rect.
pub fn ray_vs_rect(ray: Ray, position: Point, size: Size, padding: Option<Size>) -> Option<Collision> {
  // Inverse the ray.
  let inverse_direction = ray.direction.map(|i| i.recip());
  // Unwrap the padding.
  let padding = padding.unwrap_or_default();
  // Half-extents.
  let half = size / 2.0;
  // Calculate the times.
  let t_near_x = (position.x - inverse_direction.x.signum() * (half.w + padding.w) - ray.origin.x) * inverse_direction.x;
  let t_near_y = (position.y - inverse_direction.y.signum() * (half.h + padding.h) - ray.origin.y) * inverse_direction.y;
  let t_far_x = (position.x + inverse_direction.x.signum() * (half.w + padding.w) - ray.origin.x) * inverse_direction.x;
  let t_far_y = (position.y + inverse_direction.y.signum() * (half.h + padding.h) - ray.origin.y) * inverse_direction.y;
  // Early rejection.
  if t_near_x > t_far_y || t_near_y > t_far_x {
    None?
  }
  // Sort the times.
  let t_near = t_near_x.max(t_near_y);
  let t_far = t_far_x.min(t_far_y);
  // Reject if time is out of bounds.
  if t_near >= 1.0 || t_far <= 0.0 {
    None?
  }
  // Contact details.
  let contact_time = t_near.clamp(0.0, 1.0);
  let contact_normal = if t_near_x > t_near_y {
    Vector::new(-inverse_direction.x.signum(), 0.0)
  } else {
    Vector::new(0.0, -inverse_direction.y.signum())
  };
  let contact_point = ray.origin + contact_time * ray.direction;
  Some(Collision {
    contact_time: contact_time,
    contact_point: contact_point,
    contact_normal: contact_normal,
  })
}

/// Query if a moving rect intersects a stationary rect.
pub fn dynrect_vs_rect(position1: Point, size1: Size, velocity: Vector, position2: Point, size2: Size, timestep: f32) -> Option<Collision> {
  let ray = Ray::new(position1 + size1 / 2.0, velocity * timestep);
  let result = ray_vs_rect(ray, position2, size2, Some(size1 / 2.0));
  match result {
    Some(ref collision) if collision.contact_time >= 0.0 && collision.contact_time < 1.0 => {
      result
    },
    _ => None,
  }
}