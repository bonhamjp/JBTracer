use std::any::Any;

use crate::rendering::shape::Shape;
use crate::rendering::shape::ShapeType;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Matrix4x4;

pub struct Plane {
  pub id: u64,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub material: Material 
}

impl Plane {
  pub fn new(id: u64, transform: Matrix4x4, material: Material) -> Plane {
    let tmp_inverse = transform.inverse();

    Plane { 
      id: id, 
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      material: material 
    }
  }
}

impl Shape for Plane {
  fn get_id(&self) -> u64 {
    self.id
  }

  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }

  fn get_inverse(&self) -> &Matrix4x4 {
    &self.inverse
  }

  fn get_transpose(&self) -> &Matrix4x4 {
    &self.transpose
  }

  fn get_material(&self) -> &Material {
    &self.material
  }

  fn intersections(&self, ray: &Ray, world_to_container: Matrix4x4, normal_to_world: Matrix4x4) -> Vec<Intersection> {
    let transformed_point = self.inverse.mult_point(&ray.origin);
    let transformed_vector = self.inverse.mult_vector(&ray.direction);

    let transformed_ray = Ray::new(&transformed_point, &transformed_vector);

    let mut intersections: Vec<Intersection> = Vec::new();
    
    // TODO: Use global epsilon value
    // Ray parallel to plane 
    if transformed_ray.direction.y.abs() < 0.0001 {
      return intersections;
    } 

    intersections.push(Intersection::new(-transformed_ray.origin.y / transformed_ray.direction.y, self, world_to_container, normal_to_world));
    
    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_normal = Vector::new(0.0, 1.0, 0.0);
    
    let mut transformed_normal = self.transpose.mult_vector(&object_normal).normalize();
    // transformed_normal.w = 0.0;
    // transformed_normal.normalize();

    transformed_normal
  }

  fn normal_at_with_uv(&self, point: &Point, u: f64, v: f64) -> Vector {
    // Not defined
    Vector::new(0.0, 0.0, 0.0)     
  }

  fn interpolates_normals(&self) -> bool {
    false
  }

  fn get_base_type(&self) -> ShapeType {
    ShapeType::Plane
  }
}
