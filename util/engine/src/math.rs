use std::ops::{Add, Mul};

/// A point in 2-D space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  /// Create a new point.
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      x: x,
      y: y,
    }
  }
}

impl Into<[f32; 2]> for Point {
  #[inline]
  fn into(self) -> [f32; 2] {
    [self.x, self.y]
  }
}

impl From<[f32; 2]> for Point {
  #[inline]
  fn from([x, y]: [f32; 2]) -> Self {
    Self::new(x, y)
  }
}

impl Mul<Scale> for Point {
  type Output = Point;
  #[inline]
  fn mul(self, scale: Scale) -> Self::Output {
    Point::new(self.x * scale.x, self.y * scale.y)
  }
}

impl Add<Size> for Point {
  type Output = Point;
  #[inline]
  fn add(self, size: Size) -> Self::Output {
    Point::new(self.x + size.w, self.y + size.h)
  }
}

impl rstar::Point for Point {
  type Scalar = f32;
  const DIMENSIONS: usize = 2;
  fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
    Self::new(generator(0), generator(1))
  }
  fn nth(&self, index: usize) -> Self::Scalar {
    match index {
      0 => self.x,
      1 => self.y,
      _ => unreachable!(),
    }
  }
  fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      _ => unreachable!(),
    }
  }
}

/// A scale.
pub type Scale = nalgebra::Scale2<f32>;

/// A 2-D vector.
pub type Vector = nalgebra::Vector2<f32>;

/// A 4x4 matrix.
pub type Matrix4 = nalgebra::Matrix4<f32>;

/// A size.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size {
  w: f32,
  h: f32,
}

impl Size {
  /// Create a new size.
  pub fn new(w: f32, h: f32) -> Self {
    Self {
      w: w,
      h: h,
    }
  }
}

/// An axis-aligned bounding box.
pub type AABB = rstar::AABB<Point>;