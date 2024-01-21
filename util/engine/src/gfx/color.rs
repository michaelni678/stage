/// A color. Defaults to white.
#[derive(Clone, Copy)]
pub struct Color {
  red: f32,
  green: f32,
  blue: f32,
  alpha: f32,
}

impl Default for Color {
  fn default() -> Self {
    Self {
      red: 1.0,
      green: 1.0,
      blue: 1.0,
      alpha: 1.0,
    }
  }
}

impl Color {
  /// Create a new white color.
  pub fn none() -> Self {
    Self::default()
  }
  /// Create a new RGB color.
  pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
    Self {
      red: red,
      green: green,
      blue: blue,
      ..Default::default()
    }
  }
  /// Create a new white color with custom alpha.
  pub fn alpha(alpha: f32) -> Self {
    Self {
      alpha: alpha,
      ..Default::default()
    }
  }
  /// Create a new RGBA color.
  pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
    Self {
      red: red,
      green: green,
      blue: blue,
      alpha: alpha,
    }
  }
}

impl Into<[f32; 4]> for Color {
  fn into(self) -> [f32; 4] {
    [
      self.red,
      self.green,
      self.blue,
      self.alpha,
    ]
  }
}