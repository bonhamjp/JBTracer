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
  Cube,
  Cylinder,
  Cone,
  Triangle,
  SmoothTriangle,
  ConstructiveGeometry
}

pub trait Shape {
  fn get_id(&self) -> u64;

  fn get_transform(&self) -> &Matrix4x4;
  fn get_inverse(&self) -> &Matrix4x4;
  fn get_transpose(&self) -> &Matrix4x4;
  
  fn get_material(&self) -> &Material;

  fn intersections(&self, ray: &Ray, world_to_container: Matrix4x4, normal_to_world: Matrix4x4) -> Vec<Intersection>;
  
  fn normal_at(&self, point: &Point) -> Vector;
  fn normal_at_with_uv(&self, point: &Point, u: f64, v: f64) -> Vector;

  fn interpolates_normals(&self) -> bool;

  fn get_base_type(&self) -> ShapeType;
  
  fn is_eq(&self, r_hand: &Shape) -> bool {
    self.get_id() == r_hand.get_id()
  }
}
