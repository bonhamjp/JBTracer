use crate::rendering::math::Point;
use crate::rendering::math::Vector;

use crate::rendering::math::Matrix4x4;

use crate::rendering::shapes::shape::Shape;
use crate::rendering::shapes::shape::ShapeType;
use crate::rendering::shapes::shape::generate_shape_id;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

pub struct Cylinder {
  pub id: u64,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub capped: bool,
  pub minimum: f64,
  pub maximum: f64,
  pub material: Material
}

impl Cylinder {
  pub fn new(transform: Matrix4x4, capped: bool, minimum: f64, maximum: f64, material: Material) -> Cylinder {
    let tmp_inverse = transform.inverse();
    
    Cylinder { 
      id: generate_shape_id(), 
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      capped: capped,
      minimum: minimum,
      maximum: maximum, 
      material: material
    }
  }

  pub fn within_cap(&self, ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    (x * x + z * z) <= 1.0
  }
}

impl Shape for Cylinder {
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

    let a = transformed_ray.direction.x * transformed_ray.direction.x + transformed_ray.direction.z * transformed_ray.direction.z;

    // Ray is parallel to y axis if a is zero, so no collisions
    if !self.capped && a.abs() < 0.0001 { 
      return intersections;
    }

    let b = 2.0 * transformed_ray.origin.x * transformed_ray.direction.x + 2.0 * transformed_ray.origin.z * transformed_ray.direction.z;
    let c = transformed_ray.origin.x * transformed_ray.origin.x + transformed_ray.origin.z * transformed_ray.origin.z - 1.0;

    let disc = b * b - 4.0 * a * c;

    // Ray misses cylinder if discriminant less than zero
    if disc < 0.0 {
      return intersections;
    }

    let mut t_0 = (-b - disc.sqrt()) / (2.0 * a);
    let mut t_1 = (-b + disc.sqrt()) / (2.0 * a);

    if t_0 > t_1 {
      let tmp_t = t_0;

      t_0 = t_1;
      t_1 = tmp_t;
    }

    let y_0 = transformed_ray.origin.y + t_0 * transformed_ray.direction.y;
    if self.minimum < y_0 && y_0 < self.maximum {
      intersections.push(Intersection::new(t_0, self, world_to_container, normal_to_world));
    }

    let y_1 = transformed_ray.origin.y + t_1 * transformed_ray.direction.y;
    if self.minimum < y_1 && y_1 < self.maximum {
      intersections.push(Intersection::new(t_1, self, world_to_container, normal_to_world));
    }

    // If capped check for intersection within caps
    if self.capped {
      let t_c_0 = (self.minimum - transformed_ray.origin.y) / transformed_ray.direction.y;
      if self.within_cap(&transformed_ray, t_c_0) {
        intersections.push(Intersection::new(t_c_0, self, world_to_container, normal_to_world));
      }

      let t_c_1 = (self.maximum - transformed_ray.origin.y) / transformed_ray.direction.y;
      if self.within_cap(&transformed_ray, t_c_1) {
        intersections.push(Intersection::new(t_c_1, self, world_to_container, normal_to_world));          
      }
    }

    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_point = self.inverse.mult_point(point);
    
    let distance = object_point.x * object_point.x + object_point.z * object_point.z;

    let object_normal;

    // Normal from top cap
    if distance < 1.0 && object_point.y >= (self.maximum - 0.0001) {
      object_normal = Vector::new(0.0, 1.0, 0.0);

    // Normal from bottom cap
    } else if distance < 1.0 && object_point.y <= (self.minimum + 0.0001) {
      object_normal = Vector::new(0.0, -1.0, 0.0);

    // Normal from cylinder body
    } else {
      object_normal = Vector::new(object_point.x, 0.0, object_point.z).normalize();
    }

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
    ShapeType::Cylinder
  }
}
