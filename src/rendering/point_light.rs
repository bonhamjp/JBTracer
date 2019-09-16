use crate::rendering::Material;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;
use crate::math::Color;

#[derive(PartialEq)]
pub struct PointLight {
  pub intensity: Color,
  pub position: Point
}

impl PointLight {
  pub fn new(intensity: Color, position: Point) -> PointLight {
    PointLight { intensity: intensity, position: position }
  }

  pub fn default() -> PointLight {
    PointLight { intensity: Color::new(1.0, 1.0, 1.0, 1.0), position: Point::empty() }
  }

  pub fn lighting(&self, material: &Material, position: &Point, eye_v: &Vector, normal: &Vector, in_shadow: bool) -> Color {
    let effective_color = material.color.mult_color(&self.intensity);

    // direction to light source
    let light_v = self.position.subtract_point(position).normalize();

    let ambient = effective_color.mult_scalar(material.ambient);
    let mut diffuse = Color::new(0.0, 0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0, 0.0);

    // check if light is on other side of surface
    let light_dot_normal = light_v.dot(normal);
    if !in_shadow && light_dot_normal >= 0.0 {
      diffuse = effective_color.mult_scalar(material.diffuse).mult_scalar(light_dot_normal);

      // check if light reflects away from eye
      let negative_light_direction = light_v.multiply(-1.0);
      let reflect_v = negative_light_direction.reflect(&normal);
      let reflect_dot_eye = reflect_v.dot(eye_v);

      if reflect_dot_eye > 0.0 {
        let factor = reflect_dot_eye.powf(material.shininess);
        specular = self.intensity.mult_scalar(material.specular).mult_scalar(factor);
      }
    }

    ambient.add_color(&diffuse).add_color(&specular)
  }
}
