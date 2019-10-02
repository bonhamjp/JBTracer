use crate::rendering::shape::Shape;
use crate::rendering::shape::ShapeType;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Color;

use crate::math::Matrix4x4;

pub struct Sphere {
  pub id: u64,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub material: Material
}

impl Sphere {
  pub fn new(id: u64, transform: Matrix4x4, material: Material) -> Sphere {
    let tmp_inverse = transform.inverse();

    Sphere { 
      id: id, 
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      material: material
    }
  }
}

impl Shape for Sphere {
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

    let sphere_to_ray = transformed_ray.origin.subtract_point(&Point::new(0.0, 0.0, 0.0));

    let a = transformed_ray.direction.dot(&transformed_ray.direction);
    let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    // missed if discriminant negative
    if discriminant >= 0.0 {
      intersections.push(Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self, world_to_container, normal_to_world));
      intersections.push(Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self, world_to_container, normal_to_world));
    }

    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_point = self.inverse.mult_point(point);
    let object_normal = object_point.subtract_point(&Point::empty());

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
    ShapeType::Sphere
  }
}
