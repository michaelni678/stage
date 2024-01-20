use std::ops::{Add, Mul};

/// A point in 2-D space.
#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
  x: f32,
  y: f32,
}

impl Point {
  /// Create a new point.
  pub fn new(x: f32, y: f32) -> Self {
    Self { x: x, y: y }
  }
}

impl From<[f32; 2]> for Point {
  fn from([x, y]: [f32; 2]) -> Self {
    Self { x: x, y: y }
  }
}

impl From<Point> for [f32; 2] {
  fn from(point: Point) -> Self {
    [point.x, point.y]
  }
}

impl Add for Point {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl Mul<Scale> for Point {
  type Output = Self;
  fn mul(self, rhs: Scale) -> Self::Output {
    Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
    }
  }
}

/// A scale.
#[derive(Copy, Clone, Debug, Default)]
pub struct Scale {
  x: f32,
  y: f32,
}

impl Scale {
  /// Create a new scale.
  pub fn new(x: f32, y: f32) -> Self {
    Self { x: x, y: y }
  }
}

impl From<[f32; 2]> for Scale {
  fn from([x, y]: [f32; 2]) -> Self {
    Self { x: x, y: y }
  }
}

impl From<Scale> for [f32; 2] {
  fn from(scale: Scale) -> Self {
    [scale.x, scale.y]
  }
}

/// A 4x4 matrix.
pub type Matrix4 = nalgebra::Matrix4<f32>;
