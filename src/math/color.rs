extern crate num;
use num::clamp;

const COLOR_RANGE: u64 = 255;

#[derive(PartialEq)]
pub struct Color {
  pub r: f64,
  pub g: f64,
  pub b: f64,
  pub a: f64
}

impl Color {
  pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
    Color { r: r, g: g, b: b, a: a }
  }

  pub fn empty() -> Color {
    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
  }

  pub fn mult_color(&self, r_side: &Color) -> Color {
    Color::new(self.r * r_side.r, self.g * r_side.g, self.b * r_side.b, 1.0)
  }

  pub fn mult_scalar(&self, r_side: f64) -> Color {
    Color::new(self.r * r_side, self.g * r_side, self.b * r_side, 1.0)
  }

  pub fn add_color(&self, r_side: &Color) -> Color {
    Color::new(self.r + r_side.r, self.g + r_side.g, self.b + r_side.b, 1.0)
  }

  pub fn divide_color(&self, r_side: &Color) -> Color {
    let r;
    let g;
    let b;

    if r_side.r > 0.0 {
      r = self.r / r_side.r;
    } else {
      r = 0.0;
    }

    if r_side.g > 0.0 {
      g = self.g / r_side.g;
    } else {
      g = 0.0;
    }

    if r_side.b > 0.0 {
      b = self.b / r_side.b;
    } else {
      b = 0.0;
    }

    Color::new(r, g, b, 1.0)
  }

  pub fn r_display(&self) -> u64 {
    clamp((self.r * (COLOR_RANGE as f64)) as i64, 0, COLOR_RANGE as i64) as u64
  }

  pub fn g_display(&self) -> u64 {
    clamp((self.g * (COLOR_RANGE as f64)) as i64, 0, COLOR_RANGE as i64) as u64
  }

  pub fn b_display(&self) -> u64 {
    clamp((self.b * (COLOR_RANGE as f64)) as i64, 0, COLOR_RANGE as i64) as u64
  }

  pub fn a_display(&self) -> u64 {
    clamp((self.a * (COLOR_RANGE as f64)) as i64, 0, COLOR_RANGE as i64) as u64
  }

  pub fn display_values(&self) -> (u64, u64, u64, u64) {
    (self.r_display(), self.g_display(), self.b_display(), self.a_display())
  }
}
