use std::io;
use std::io::prelude::*;

pub mod math;
pub mod rendering;

fn main() {
    // let canvas = rendering::Canvas::new(1080, 720);
    
    let mut canvas = rendering::Canvas::new(8, 8);

    canvas.color_pixel(1, 2, math::Color::new(1.0, 0.0, 0.0, 1.0));
    canvas.color_pixel(1, 5, math::Color::new(1.0, 0.0, 0.0, 1.0));
    canvas.color_pixel(5, 1, math::Color::new(0.0, 0.0, 1.0, 1.0));
    canvas.color_pixel(5, 6, math::Color::new(0.0, 0.0, 1.0, 1.0));
    canvas.color_pixel(6, 2, math::Color::new(0.0, 1.0, 0.0, 1.0));
    canvas.color_pixel(6, 3, math::Color::new(0.0, 1.0, 0.0, 1.0));
    canvas.color_pixel(6, 4, math::Color::new(0.0, 1.0, 0.0, 1.0));
    canvas.color_pixel(6, 5, math::Color::new(0.0, 1.0, 0.0, 1.0));

    canvas.save_image();
}
