use std::ops::{Add, Div, Mul, Sub};

/// A point in 2-D space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  /// Create a new point.
  pub fn new(x: f32, y: f32) -> Self {
    Self { x: x, y: y }
  }
  /// Check if any of the values are NaN.
  pub fn has_nan(&self) -> bool {
    self.x.is_nan() || self.y.is_nan()
  }
}

impl From<Point> for [f32; 2] {
  fn from(point: Point) -> Self {
    [point.x, point.y]
  }
}

impl From<[f32; 2]> for Point {
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

impl Add for Point {
  type Output = Point;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl Sub for Point {
  type Output = Point;
  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl Add<Size> for Point {
  type Output = Point;
  #[inline]
  fn add(self, size: Size) -> Self::Output {
    Point::new(self.x + size.w, self.y + size.h)
  }
}

impl Sub<Size> for Point {
  type Output = Point;
  #[inline]
  fn sub(self, size: Size) -> Self::Output {
    Point::new(self.x - size.w, self.y - size.h)
  }
}

impl Add<Vector> for Point {
  type Output = Point;
  #[inline]
  fn add(self, vector: Vector) -> Self::Output {
    Point::new(self.x + vector.x, self.y + vector.y)
  }
}

impl Mul<Vector> for Point {
  type Output = Point;
  #[inline]
  fn mul(self, vector: Vector) -> Self::Output {
    Point::new(self.x * vector.x, self.y * vector.y)
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
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Size {
  pub w: f32,
  pub h: f32,
}

impl Size {
  /// Create a new size.
  pub fn new(w: f32, h: f32) -> Self {
    Self { w: w, h: h }
  }
}

impl From<Size> for [f32; 2] {
  fn from(size: Size) -> Self {
    [size.w, size.h]
  }
}

impl From<[f32; 2]> for Size {
  fn from([x, y]: [f32; 2]) -> Self {
    Self::new(x, y)
  }
}

impl From<Size> for Scale {
  fn from(size: Size) -> Self {
    Self::new(size.w, size.h)
  }
}

impl Add for Size {
  type Output = Size;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Size::new(self.w + rhs.w, self.h + rhs.h)
  }
}

impl Div<f32> for Size {
  type Output = Size;
  #[inline]
  fn div(self, rhs: f32) -> Self::Output {
    Self::new(self.w / rhs, self.h / rhs)
  }
}

/// An axis-aligned bounding box.
pub type AABB = rstar::AABB<Point>;

/// A ray.
pub struct Ray {
  pub origin: Point,
  pub direction: Vector,
}

impl Ray {
  /// Create a new ray.
  pub fn new(origin: impl Into<Point>, direction: impl Into<Vector>) -> Self {
    Self {
      origin: origin.into(),
      direction: direction.into(),
    }
  }
}
