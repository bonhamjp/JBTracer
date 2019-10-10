use crate::rendering::math::Point;

use crate::rendering::math::Color;

use crate::rendering::math::Matrix4x4;

use crate::rendering::shapes::shape::Shape;

// const VACUUM_REFRACTIVE_INDEX: f64 = 1.0;
// const AIR_REFRACTIVE_INDEX: f64 = 1.00029;
// const WATER_REFRACTIVE_INDEX: f64 = 1.333;
// const GLASS_REFRACTIVE_INDEX: f64 = 1.52;
// const DIAMOND_REFRACTIVE_INDEX: f64 = 2.417;

pub struct Material {
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
  pub reflectiveness: f64,
  pub transparency: f64,
  pub refractive_index: f64,
  pub color_1: Color,
  pub color_2: Color,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub pattern_func: fn(material: &Material, object: &dyn Shape, position: &Point) -> Color
}

impl Material {
  pub fn solid(
    ambient: f64, 
    diffuse: f64, 
    specular: f64, 
    shininess: f64, 
    reflectiveness: f64, 
    transparency: f64, 
    refractive_index: f64,
    color: Color,
    transform: Matrix4x4, 
  ) -> Material {
    
    Material { 
      ambient: ambient, 
      diffuse: diffuse, 
      specular: specular,
      reflectiveness: reflectiveness,
      shininess: shininess,
      transparency: transparency,
      refractive_index: refractive_index,
      color_1: color,
      color_2: color,
      transform: transform,
      inverse: transform.inverse(),
      pattern_func: solid_pattern_func
    }
  }

  pub fn checkered(
    ambient: f64, 
    diffuse: f64, 
    specular: f64, 
    shininess: f64, 
    reflectiveness: f64, 
    transparency: f64, 
    refractive_index: f64,
    color_1: Color,
    color_2: Color,
    transform: Matrix4x4
  ) -> Material {

    Material { 
      ambient: ambient, 
      diffuse: diffuse, 
      specular: specular,
      reflectiveness: reflectiveness,
      shininess: shininess,
      transparency: transparency,
      refractive_index: refractive_index,
      color_1: color_1,
      color_2: color_2,
      transform: transform,
      inverse: transform.inverse(),
      pattern_func: checker_pattern_func
    }
  }

  pub fn striped(
    ambient: f64, 
    diffuse: f64, 
    specular: f64, 
    shininess: f64, 
    reflectiveness: f64, 
    transparency: f64, 
    refractive_index: f64,
    color_1: Color,
    color_2: Color,
    transform: Matrix4x4
  ) -> Material {

    Material { 
      ambient: ambient, 
      diffuse: diffuse, 
      specular: specular,
      reflectiveness: reflectiveness,
      shininess: shininess,
      transparency: transparency,
      refractive_index: refractive_index,
      color_1: color_1,
      color_2: color_2,
      transform: transform,
      inverse: transform.inverse(),
      pattern_func: stripe_pattern_func
    }
  }

  pub fn ringed(
    ambient: f64, 
    diffuse: f64, 
    specular: f64, 
    shininess: f64, 
    reflectiveness: f64, 
    transparency: f64, 
    refractive_index: f64,
    color_1: Color,
    color_2: Color,
    transform: Matrix4x4
  ) -> Material {

    Material { 
      ambient: ambient, 
      diffuse: diffuse, 
      specular: specular,
      reflectiveness: reflectiveness,
      shininess: shininess,
      transparency: transparency,
      refractive_index: refractive_index,
      color_1: color_1,
      color_2: color_2,
      transform: transform,
      inverse: transform.inverse(),
      pattern_func: ring_pattern_func
    }
  }

  pub fn gradient(
    ambient: f64, 
    diffuse: f64, 
    specular: f64, 
    shininess: f64, 
    reflectiveness: f64, 
    transparency: f64, 
    refractive_index: f64,
    color_1: Color,
    color_2: Color,
    transform: Matrix4x4
  ) -> Material {

    Material { 
      ambient: ambient, 
      diffuse: diffuse, 
      specular: specular,
      reflectiveness: reflectiveness,
      shininess: shininess,
      transparency: transparency,
      refractive_index: refractive_index,
      color_1: color_1,
      color_2: color_2,
      transform: transform,
      inverse: transform.inverse(),
      pattern_func: gradient_pattern_func
    }
  }

  pub fn convert_point(&self, object: &dyn Shape, position: &Point) -> Point {
    let object_point = object.get_inverse().mult_point(position);
    
    self.inverse.mult_point(&object_point)
  }

  pub fn color_at(&self, object: &dyn Shape, position: &Point) -> Color {
    (self.pattern_func)(self, object, position)
  }
}

pub fn solid_pattern_func(material: &Material, _object: &dyn Shape, _position: &Point) -> Color {
  material.color_1
}

pub fn checker_pattern_func(material: &Material, object: &dyn Shape, position: &Point) -> Color {
  let pattern_point = material.convert_point(object, position);

  let summed_floor = pattern_point.x.round() + pattern_point.y.round() + pattern_point.z.round();
  if (summed_floor as u64) % 2 == 0 {
    material.color_1
  } else {
    material.color_2
  }
}

pub fn stripe_pattern_func(material: &Material, object: &dyn Shape, position: &Point) -> Color {
  let pattern_point = material.convert_point(object, position);

  let adjusted_x;
  
  if pattern_point.x < 0.0 {
    adjusted_x = (((pattern_point.x) * -1.0) + 1.0) as u64;
  } else {
    adjusted_x = pattern_point.x as u64;
  }
  
  if adjusted_x % 2 == 0 {
    material.color_1
  } else {
    material.color_2
  }
}

pub fn ring_pattern_func(material: &Material, object: &dyn Shape, position: &Point) -> Color {
  let pattern_point = material.convert_point(object, position);
  
  if ((pattern_point.x * pattern_point.x + pattern_point.z * pattern_point.z).sqrt().floor() as u64) % 2 != 0 {
    material.color_1
  } else {
    material.color_2
  }
}

pub fn gradient_pattern_func(material: &Material, object: &dyn Shape, position: &Point) -> Color {
  let pattern_point = material.convert_point(object, position);
  
  let remainder = pattern_point.x - pattern_point.x.floor();

  let distance = material.color_2.subtract_color(&material.color_1);

  material.color_1.add_color(&distance.mult_scalar(remainder))
}
