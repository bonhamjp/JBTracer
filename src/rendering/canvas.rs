use std::io::prelude::*;
use std::fs::File;

extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

use crate::math::Color;

const MAX_PPM_LINE_WIDTH: usize = 70; 
const CLEAR_COLOR: f64 = 1.0;

pub struct Canvas {
  pub width: u64,
  pub height: u64,
  pub color_buffer: Vec<Color>
}

impl Canvas {
  pub fn new(width: u64, height: u64) -> Canvas {
    let mut color_buffer: Vec<Color> = Vec::new();

    let buffer_size = width * height;
    for i in 0..buffer_size {
      let color = Color::new(CLEAR_COLOR, CLEAR_COLOR, CLEAR_COLOR, 1.0);
      color_buffer.push(color);
    }

    Canvas { width: width, height: height, color_buffer: color_buffer }
  }

  pub fn color_pixel(&mut self, row: u64, column: u64, color: Color) {
    // TODO: Bounds checking
    self.color_buffer[((row * self.width) + column) as usize] = color
  }

  pub fn pixel_color(&self, row: u64, column: u64) -> &Color {
    self.color_buffer.get(((row * self.width) + column) as usize).unwrap()
  }

  pub fn image_output(&self) -> Vec<String> {
    let mut image_data_lines: Vec<String> = Vec::new();

    // ppm version
    image_data_lines.push(String::from("P3\n"));
    // pixel width x height
    image_data_lines.push(format!("{} {}\n", self.width, self.height));
    // colr value range
    image_data_lines.push(String::from("255\n"));

    for i in 0..self.height {
      let mut color_row = String::new();

      for j in 0..self.width {
        let (display_r, display_g, display_b, _) = self.pixel_color(i, j).display_values();

        let r_string = &format!("{} ", display_r).to_string();
        if (color_row.chars().count() + r_string.chars().count() + 1 > MAX_PPM_LINE_WIDTH) {
          color_row.push_str("\n");
          image_data_lines.push(color_row);

          color_row = String::new();
        }
        color_row.push_str(r_string);

        let g_string = &format!("{} ", display_g).to_string();
        if (color_row.chars().count() + g_string.chars().count() + 1 > MAX_PPM_LINE_WIDTH) {
          color_row.push_str("\n");
          image_data_lines.push(color_row);

          color_row = String::new();
        }
        color_row.push_str(g_string);

        let b_string = &format!("{} ", display_b).to_string();
        if (color_row.chars().count() + b_string.chars().count() + 1 > MAX_PPM_LINE_WIDTH) {
          color_row.push_str("\n");
          image_data_lines.push(color_row);

          color_row = String::new();
        }
        color_row.push_str(b_string);
      }

      color_row.push_str("\n");
      image_data_lines.push(color_row);
    }

    // requires trailing newline
    image_data_lines.push(String::from(""));

    image_data_lines
  }

  pub fn save_image(&self) -> std::io::Result<()> {
    let current_time_string = Utc::now().format("%a-%b-%e-%s-%Y");

    let file_name = format!("output/{}_{}_{}.ppm", current_time_string, self.width, self.height);

    let mut file = File::create(file_name).unwrap();

    for data_line in &self.image_output() {
      file.write_all(data_line.as_bytes());
    }

    Ok(())
  }
}
