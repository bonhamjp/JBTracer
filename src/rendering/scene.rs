use std::f64;

use crate::rendering::Canvas;

use crate::rendering::Camera;

use crate::rendering::PointLight;   

use crate::rendering::shape::Shape;
use crate::rendering::Sphere;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;
use crate::rendering::Computations;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Color;

use crate::math::Matrix4x4;

pub struct Scene<'a> {
  pub camera: Camera,
  pub lights: Vec<PointLight>,
  pub objects: Vec<&'a dyn Shape>
}

impl<'a> Scene<'a> {
  pub fn new(camera: Camera, lights: Vec<PointLight>, objects: Vec<&'a dyn Shape>) -> Scene {
    Scene { camera: camera, lights: lights, objects: objects }
  }

  pub fn empty() -> Scene<'a> {
    let empty_camera = Camera::new(0, 0, f64::consts::PI / 2.0, Matrix4x4::identity()); 
    Scene { camera: empty_camera, lights: Vec::new(), objects: Vec::new() }
  }

  pub fn intersect<'b>(&self, ray: &'b Ray) -> Vec<Intersection<'a>> {
    let mut intersections = Vec::new();

    for object in &self.objects {
      let mut new_intersections = object.intersections(&ray);

      intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    }

    intersections 
  }

  pub fn is_shadowed(&self, world_position: &Point, light_position: &Point) -> bool {
    let towards_light = light_position.subtract_point(&world_position);
    let distance = towards_light.magnitude();
    let direction = towards_light.normalize(); 

    let ray = Ray::new(&world_position, &direction);

    let intersections = self.intersect(&ray);

    let hit = Intersection::get_hit(&intersections);
    if let Some(intersection) = Intersection::get_hit(&intersections) {
      return intersection.t < distance;

    } else {
      return false;

    }
  }

  pub fn shade_hit(&self, computations: &Computations) -> Color {
    let mut shaded_color = Color::new(0.0, 0.0, 0.0, 1.0);

    for light in &self.lights {
      // TODO: Check if in shadow of light
      let shadowed = self.is_shadowed(&computations.over_point, &light.position); 

      let light_color = light.lighting(computations.object.get_material(), &computations.point, &computations.eye_v, &computations.normal, shadowed);
      shaded_color = shaded_color.add_color(&light_color)
    }

    shaded_color
  }

  pub fn color_at(&self, ray: &Ray) -> Color {
    let intersections = self.intersect(ray);

    let hit = Intersection::get_hit(&intersections);

    if let Some(intersection) = Intersection::get_hit(&intersections) {
      let computations = Computations::new(&intersection, &ray);

      return self.shade_hit(&computations);
    } else {
      Color::new(0.0, 0.0, 0.0, 1.0)
    }
  }

  pub fn render(&self) -> Canvas {
    let mut canvas = Canvas::new(self.camera.horizontal_size, self.camera.vertical_size);

    for y in 0..self.camera.vertical_size {
      for x in 0..self.camera.horizontal_size {
        // // No Anti-Aliasing
        // let ray = self.camera.ray_for_pixel(x, y);
        // let color = self.color_at(&ray);

        // Anti-Aliasing
        let rays = self.camera.rays_for_pixel_x16_sample_rate(x, y);
        let mut color = Color::new(0.0, 0.0, 0.0, 1.0);
        let mut color_count = 0.0;        
        for ray in rays {
          color = color.add_color(&self.color_at(&ray));

          color_count = color_count + 1.0;
        }
        color = color.divide_color(&Color::new(color_count, color_count, color_count, 1.0));

        canvas.color_pixel(y, x, color);
      }
    }

    canvas
  }
}
