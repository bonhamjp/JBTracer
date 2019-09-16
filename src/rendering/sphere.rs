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
pub struct Sphere {
  pub transform: Matrix4x4,
  pub material: Material
}

impl Sphere {
  pub fn new(transform: Matrix4x4, material: Material) -> Sphere {
    Sphere { transform: transform, material: material }
  }
  
  pub fn default() -> Sphere {
    Sphere { transform: Matrix4x4::identity(), material: Material::default() }
  }
}

impl Shape for Sphere {
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

    let sphere_to_ray = transformed_ray.origin.subtract_point(&Point::new(0.0, 0.0, 0.0));

    let a = transformed_ray.direction.dot(&transformed_ray.direction);
    let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    // missed if discriminant negative
    if discriminant >= 0.0 {
      let sphere = self as &Sphere;

      intersections.push(Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), sphere));
      intersections.push(Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), sphere));
    }

    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_point = self.transform.inverse().mult_point(point);
    let object_normal = object_point.subtract_point(&Point::empty());

    let mut world_normal = self.transform.inverse().transpose().mult_vector(&object_normal);
    world_normal.w = 0.0;

    world_normal.normalize()
  }

  fn get_base_type(&self) -> ShapeType {
    ShapeType::Sphere
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn is_eq(&self, r_hand: &Shape) -> bool {
    if self.get_base_type() != r_hand.get_base_type() {
      return false;
    } else {
      return self as *const _ == r_hand.as_any().downcast_ref::<Sphere>().unwrap() as *const _
    }
  }
}
