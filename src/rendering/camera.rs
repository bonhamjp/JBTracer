use crate::rendering::Ray;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Matrix4x4;

pub struct Camera {
  pub horizontal_size: u64,
  pub vertical_size: u64,
  pub field_of_view: f64,
  pub aspect_ratio: f64,
  pub half_width: f64,
  pub half_height: f64,
  pub pixel_size: f64,
  pub transform: Matrix4x4
}

impl Camera {
  pub fn new(horizontal_size: u64, vertical_size: u64, field_of_view: f64, transform: Matrix4x4) -> Camera {
    let half_view = (field_of_view / 2.0).tan();
    let aspect_ratio = (horizontal_size as f64) / (vertical_size as f64);

    let half_width;
    let half_height;

    if aspect_ratio >= 1.0 {
      half_width = half_view;
      half_height = half_view / aspect_ratio;
    } else {
      half_width = half_view * aspect_ratio;
      half_height = half_view;
    }

    let pixel_size = (half_width * 2.0) / (horizontal_size as f64);
    
    Camera { 
      horizontal_size: horizontal_size, 
      vertical_size: vertical_size, 
      field_of_view: field_of_view,
      aspect_ratio: aspect_ratio,
      half_width: half_width,
      half_height: half_height,
      pixel_size: pixel_size,
      transform: transform
    }
  }

  pub fn ray_for_pixel(&self, x: u64, y: u64) -> Ray {
    let x_offset = ((x as f64) + 0.5) * self.pixel_size;
    let y_offset = ((y as f64) + 0.5) * self.pixel_size;

    let world_x = self.half_width - x_offset;
    let world_y = self.half_height - y_offset;

    let inverse_transform = self.transform.inverse();

    // canvas at z = -1
    let pixel = inverse_transform.mult_point(&Point::new(world_x, world_y, -1.0));
    let origin = inverse_transform.mult_point(&Point::new(0.0, 0.0, 0.0));
    let direction = pixel.subtract_point(&origin).normalize();

    Ray::new(&origin, &direction)
  }

  pub fn rays_for_pixel_x4_sample_rate(&self, x: u64, y: u64) -> Vec<Ray> {
    let mut rays = Vec::new();

    let inverse_transform = self.transform.inverse();
    let origin_point = inverse_transform.mult_point(&Point::new(0.0, 0.0, 0.0));

    let mut pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.30) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.30) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.60) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.30) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.30) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.60) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.60) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.60) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    rays
  }

  pub fn rays_for_pixel_x16_sample_rate(&self, x: u64, y: u64) -> Vec<Ray> {
    let mut rays = Vec::new();

    let inverse_transform = self.transform.inverse();
    let origin_point = inverse_transform.mult_point(&Point::new(0.0, 0.0, 0.0));

    let mut pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.20) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.20) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.40) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.20) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.60) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.20) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.80) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.20) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    let mut pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.20) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.40) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.40) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.40) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.60) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.40) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.80) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.40) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    let mut pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.20) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.60) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.40) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.60) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.60) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.60) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.80) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.60) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    let mut pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.20) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.80) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.40) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.80) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.60) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.80) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    pixel = inverse_transform.mult_point(
      &Point::new(
        self.half_width - ((x as f64) + 0.80) * self.pixel_size, 
        self.half_height - ((y as f64) + 0.80) * self.pixel_size, 
        -1.0
      )
    );    
    rays.push(Ray::new(&origin_point, &pixel.subtract_point(&origin_point).normalize()));

    rays
  }
}