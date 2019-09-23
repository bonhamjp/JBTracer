use crate::rendering::shape::Shape;

use crate::rendering::Material;

use crate::math::Point;

use crate::math::Color;

use crate::math::Matrix4x4;

pub trait Pattern {
  fn color_at(&self, point: &Point) -> Color;

  fn get_transform(&self) -> &Matrix4x4;

  fn convert_point(&self, object: &dyn Shape, position: &Point) -> Point {
    let material = object.get_material();
    
    let object_point = object.get_transform().inverse().mult_point(position);
    let pattern_point = material.pattern.get_transform().inverse().mult_point(&object_point);

    pattern_point
  }
}
