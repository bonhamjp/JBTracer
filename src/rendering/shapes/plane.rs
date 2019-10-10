use crate::rendering::math::Point;
use crate::rendering::math::Vector;

use crate::rendering::math::Matrix4x4;

use crate::rendering::shapes::shape::Shape;
use crate::rendering::shapes::shape::ShapeType;
use crate::rendering::shapes::shape::generate_shape_id;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

pub struct Plane {
  pub id: u64,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub material: Material 
}

impl Plane {
  pub fn new(transform: Matrix4x4, material: Material) -> Plane {
    let tmp_inverse = transform.inverse();

    Plane { 
      id: generate_shape_id(), 
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
    
    // Ray parallel to plane 
    if transformed_ray.direction.y.abs() < 0.0001 {
      return intersections;
    } 

    intersections.push(Intersection::new(-transformed_ray.origin.y / transformed_ray.direction.y, self, world_to_container, normal_to_world));
    
    intersections
  }

  fn normal_at(&self, _point: &Point) -> Vector {
    let object_normal = Vector::new(0.0, 1.0, 0.0);
    
    self.transpose.mult_vector(&object_normal).normalize()
  }

  fn normal_at_with_uv(&self, _point: &Point, _u: f64, _v: f64) -> Vector {
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
