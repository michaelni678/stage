use std::ops::{Add, Mul};

/// An X and Y in 2-D space.
#[derive(Copy, Clone, Debug, Default)]
pub struct XY {
  x: f32,
  y: f32,
}

impl XY {
  /// Create a new XY.
  pub const fn new(x: f32, y: f32) -> Self {
    Self { x: x, y: y }
  }
  /// Scale an XY by another XY.
  pub fn scale(&self, other: XY) -> Self {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
    }
  }
}

impl From<[f32; 2]> for XY {
  fn from([x, y]: [f32; 2]) -> Self {
    Self { x: x, y: y }
  }
}

impl From<XY> for [f32; 2] {
  fn from(xy: XY) -> Self {
    [xy.x, xy.y]
  }
}

impl Add for XY {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl Mul<f32> for XY {
  type Output = Self;
  fn mul(self, scalar: f32) -> Self::Output {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
    }
  }
}

/// A point in 2-D space.
pub type Point = XY;

/// A scale.
pub type Scale = XY;

/// A 4x4 matrix.
pub type Matrix4 = nalgebra::Matrix4<f32>;
