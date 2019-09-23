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

pub struct Plane<'a> {
  pub id: u64,
  pub transform: Matrix4x4,
  pub material: Material<'a>
}

impl<'a> Plane<'a> {
  pub fn new(id: u64, transform: Matrix4x4, material: Material<'a>) -> Plane {
    Plane { id: id, transform: transform, material: material }
  }
}

impl Shape for Plane<'_> {
  fn get_id(&self) -> u64 {
    self.id
  }

  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }

  fn get_material(&self) -> &Material {
    &self.material
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
}
