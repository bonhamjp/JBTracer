use std::f64;

use crate::rendering::math::tuple::Tuple;
use crate::rendering::math::Point;

use crate::rendering::math::Color;

use crate::rendering::Canvas;

use crate::rendering::Camera;

use crate::rendering::PointLight;   

use crate::rendering::Container;

use crate::rendering::Ray;
use crate::rendering::Intersection;
use crate::rendering::Computations;

const RAY_CAST_DEPTH: u32 = 128;

pub struct Scene<'a> {
  pub camera: Camera,
  pub lights: Vec<PointLight>,
  pub containers: Vec<Container<'a>>
}

impl<'a> Scene<'a> {
  pub fn new(camera: Camera, lights: Vec<PointLight>, containers: Vec<Container<'a>>) -> Scene<'a> {
    Scene { 
      camera: camera, 
      lights: lights, 
      containers: containers
    }
  }

  pub fn intersect<'b>(&self, ray: &'b Ray) -> Vec<Intersection<'a>> {
    let mut intersections = Vec::new();

    for container in &self.containers {
      let mut new_intersections = container.intersect(ray);

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

    if let Some(intersection) = Intersection::get_hit(&intersections) {
      return intersection.t < distance;

    } else {
      return false;

    }
  }

  pub fn shade_hit(&self, computations: &Computations, remaining_casts: u32) -> Color {
    let mut shaded_color = Color::new(0.0, 0.0, 0.0, 1.0);

    // Always return black if maximum level of cast rays reached
    if remaining_casts <= 0 {
      return shaded_color;
    }

    for light in &self.lights {
      let shadowed = self.is_shadowed(&computations.over_point, &light.position); 

      let light_color = light.lighting(computations.object, &computations.point, &computations.eye_v, &computations.normal, shadowed);
      shaded_color = shaded_color.add_color(&light_color)
    }

    let reflected_color = self.reflected_color(computations, remaining_casts - 1);
    let refracted_color = self.refracted_color(computations, remaining_casts - 1);

    let material = computations.object.get_material(); 
    if material.reflectiveness > 0.0 && material.transparency > 0.0 {
      let reflectance = self.schlick(computations);

      let reflected_fresnel = reflected_color.mult_scalar(reflectance);
      let refracted_fresnel = refracted_color.mult_scalar(1.0 - reflectance);

      return shaded_color.add_color(&reflected_fresnel).add_color(&refracted_fresnel);
    } else {
      return shaded_color.add_color(&reflected_color).add_color(&refracted_color); 
    }
  }

  pub fn color_at(&self, ray: &Ray, remaining_casts: u32) -> Color {
    let intersections = self.intersect(ray);

    if let Some(intersection) = Intersection::get_hit(&intersections) {
      let computations = Computations::new(&intersection, &ray, &intersections);

      return self.shade_hit(&computations, remaining_casts);
    } else {
      Color::new(0.0, 0.0, 0.0, 1.0)
    }
  }

  pub fn reflected_color(&self, computations: &Computations, remaining_casts: u32) -> Color {
    if computations.object.get_material().reflectiveness <= 0.0 {
      return Color::new(0.0, 0.0, 0.0, 1.0);
    } else {
      let reflection_ray = Ray::new(&computations.over_point, &computations.reflect_v);

      let reflection_color = self.color_at(&reflection_ray, remaining_casts);

      return reflection_color.mult_scalar(computations.object.get_material().reflectiveness);
    }
  }

  pub fn refracted_color(&self, computations: &Computations, remaining_casts: u32) -> Color {
    if computations.object.get_material().transparency <= 0.0 {
      return Color::new(0.0, 0.0, 0.0, 1.0);
    } else {
      let n_ratio = computations.n1 / computations.n2;

      // Dot calculates cosine of eye and normal vectors
      let cos_i = computations.eye_v.dot(&computations.normal);

      // Sin(theta_t)^2
      let sin2_t = (n_ratio * n_ratio) * (1.0 - (cos_i * cos_i));

      // Total internal reflection if sin2_t > 1
      if sin2_t > 1.0 {
        return Color::new(0.0, 0.0, 0.0, 1.0);
      }

      // Cos(theta_t)
      let cos_t = (1.0 - sin2_t).sqrt();

      // Direction of refracted ray
      let direction = computations.normal.multiply(n_ratio * cos_i - cos_t).subtract_vector(&computations.eye_v.multiply(n_ratio));

      let refraction_ray = Ray::new(&computations.under_point, &direction);

      let refracted_color = self.color_at(&refraction_ray, remaining_casts);
    
      return refracted_color.mult_scalar(computations.object.get_material().transparency);
    }
  }

  pub fn schlick(&self, computations: &Computations) -> f64 {
    // Dot calculates cosine of eye and normal vectors
    let mut cos = computations.eye_v.dot(&computations.normal);
    
    // Total internal reflection is only possible if n1 is larger than n2
    if computations.n1 > computations.n2 {
      let n_ratio = computations.n1 / computations.n2;

      // Sin(theta_t)^2
      let sin2_t = (n_ratio * n_ratio) * (1.0 - (cos * cos));

      // Total internal reflection if sin2_t > 1
      if sin2_t > 1.0 {
        return 1.0;
      }

      // Cos(theta_t)
      let cos_t = (1.0 - sin2_t).sqrt();

      cos = cos_t;
    }

    let c_0 = (computations.n1 - computations.n2) / (computations.n1 + computations.n2);
    let r_0 = c_0 * c_0;

    let sub_cos = 1.0 - cos;
    r_0 + (1.0 - r_0) * (sub_cos * sub_cos * sub_cos * sub_cos * sub_cos)
  }

  pub fn render(&self) -> Canvas {
    let mut canvas = Canvas::new(self.camera.horizontal_size, self.camera.vertical_size);

    for y in 0..self.camera.vertical_size {
      for x in 0..self.camera.horizontal_size {
        // No Anti-Aliasing
        let ray = self.camera.ray_for_pixel(x, y);
        let color = self.color_at(&ray, RAY_CAST_DEPTH);

        // // Anti-Aliasing
        // let rays = self.camera.rays_for_pixel_x4_sample_rate(x, y);
        // let mut color = Color::new(0.0, 0.0, 0.0, 1.0);
        // let mut color_count = 0.0;        
        // for ray in rays {
        //   color = color.add_color(&self.color_at(&ray, RAY_CAST_DEPTH));

        //   color_count = color_count + 1.0;
        // }
        // color = color.divide_color(&Color::new(color_count, color_count, color_count, 1.0));

        canvas.color_pixel(y, x, color);
      }
    }

    canvas
  }
}
