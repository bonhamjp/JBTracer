use std::any::Any;
use std::ptr;

use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::rendering::Material;

use crate::math::Matrix4x4;

use crate::math::Point;
use crate::math::Vector;

#[derive(PartialEq)]
pub enum ShapeType {
  Sphere,
  Plane,
  Cube
}

pub trait Shape {
  fn get_transform(&self) -> &Matrix4x4;
  fn set_transform(&mut self, transform: Matrix4x4);

  fn get_material(&self) -> &Material;
  fn set_material(&mut self, material: Material); 

  fn intersections(&self, ray: &Ray) -> Vec<Intersection>;
  fn normal_at(&self, point: &Point) -> Vector;

  fn get_base_type(&self) -> ShapeType;
  fn as_any(&self) -> &dyn Any;
  fn is_eq(&self, r_hand: &Shape) -> bool;
}
