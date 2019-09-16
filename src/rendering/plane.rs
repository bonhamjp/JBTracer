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

#[derive(PartialEq)]
pub struct Plane {
  pub transform: Matrix4x4,
  pub material: Material
}

impl Plane {
  pub fn new(transform: Matrix4x4, material: Material) -> Plane {
    Plane { transform: transform, material: material }
  }
  
  pub fn default() -> Plane {
    Plane { transform: Matrix4x4::identity(), material: Material::default() }
  }
}

impl Shape for Plane {
  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }

  fn set_transform(&mut self, transform: Matrix4x4) {
    self.transform = transform
  }

  fn get_material(&self) -> &Material {
    &self.material
  }

  fn set_material(&mut self, material: Material) {
    self.material = material
  }

  fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
    // TODO: Make methods for conveniently creating new points and vectors from transformation
    let (transformed_origin_x, transformed_origin_y, transformed_origin_z, _) = self.transform.inverse().mult4x1(&ray.origin);
    let (transformed_direction_x, transformed_direction_y, transformed_direction_z, _) = self.transform.inverse().mult4x1(&ray.direction);

    let transformed_ray = Ray::new(
      &Point::new(transformed_origin_x, transformed_origin_y, transformed_origin_z), 
      &Vector::new(transformed_direction_x, transformed_direction_y, transformed_direction_z)
    );

    let mut intersections: Vec<Intersection> = Vec::new();
    
    // TODO: Use global epsilon value
    // Ray parallel to plane 
    if transformed_ray.direction.y.abs() < 0.0001 {
      return intersections;
    } 

    intersections.push(Intersection::new(-transformed_ray.origin.y / transformed_ray.direction.y, self as &Plane));
    
    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_normal = Vector::new(0.0, 1.0, 0.0);

    let mut world_normal = self.transform.inverse().transpose().mult_vector(&object_normal);
    world_normal.w = 0.0;

    world_normal.normalize()
  }

  fn get_base_type(&self) -> ShapeType {
    ShapeType::Plane
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn is_eq(&self, r_hand: &Shape) -> bool {
    if self.get_base_type() != r_hand.get_base_type() {
      return false;
    } else {
      return self as *const _ == r_hand.as_any().downcast_ref::<Plane>().unwrap() as *const _
    }
  }
}
