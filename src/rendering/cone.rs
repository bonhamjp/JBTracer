use crate::rendering::shape::Shape;
use crate::rendering::shape::ShapeType;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Matrix4x4;

pub struct Cone {
  pub id: u64,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub capped: bool,
  pub minimum: f64,
  pub maximum: f64,
  pub material: Material
}

impl Cone {
  pub fn new(id: u64, transform: Matrix4x4, capped: bool, minimum: f64, maximum: f64, material: Material) -> Cone {
    let tmp_inverse = transform.inverse();

    Cone { 
      id: id, 
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      capped: capped,
      minimum: minimum,
      maximum: maximum, 
      material: material
    }
  }

  pub fn within_cap(&self, ray: &Ray, t: f64, radius: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    (x * x + z * z) <= radius
  }
}

impl Shape for Cone {
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

    let a = transformed_ray.direction.x * transformed_ray.direction.x - transformed_ray.direction.y * transformed_ray.direction.y + transformed_ray.direction.z * transformed_ray.direction.z;

    let b = 2.0 * transformed_ray.origin.x * transformed_ray.direction.x - 2.0 * transformed_ray.origin.y * transformed_ray.direction.y + 2.0 * transformed_ray.origin.z * transformed_ray.direction.z;
    let c = transformed_ray.origin.x * transformed_ray.origin.x - transformed_ray.origin.y * transformed_ray.origin.y + transformed_ray.origin.z * transformed_ray.origin.z;

    // Missed if both a and b are zero
    if a.abs() < 0.0001 && b.abs() < 0.0001 {
      return intersections;
    
    // Only one hit if a zero
    } else if a.abs() < 0.0001 {
      let t = -c / (2.0 * b);

      intersections.push(Intersection::new(t, self, world_to_container, normal_to_world));

    // Normal hit
    } else {
      let disc = b * b - 4.0 * a * c;

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
    }

    // If capped check for intersection within caps
    if self.capped {
    let t_c_0 = (self.minimum - transformed_ray.origin.y) / transformed_ray.direction.y;
    if self.within_cap(&transformed_ray, t_c_0, self.minimum.abs()) {
      intersections.push(Intersection::new(t_c_0, self, world_to_container, normal_to_world));
    }

    let t_c_1 = (self.maximum - transformed_ray.origin.y) / transformed_ray.direction.y;
    if self.within_cap(&transformed_ray, t_c_1, self.maximum.abs()) {
      intersections.push(Intersection::new(t_c_1, self, world_to_container, normal_to_world));          
    }
    }

    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_point = self.inverse.mult_point(point);
    
    let distance = object_point.x * object_point.x + object_point.z * object_point.z;

    let mut object_normal;

    // Normal from top cap
    if distance < 1.0 && object_point.y >= (self.maximum - 0.0001) {
      return Vector::new(0.0, 1.0, 0.0);

    // Normal from bottom cap
    } else if distance < 1.0 && object_point.y <= (self.minimum + 0.0001) {
      object_normal = Vector::new(0.0, -1.0, 0.0);

    // Normal from cone body
    } else {
      let mut y = (object_point.x * object_point.x + object_point.z * object_point.z).sqrt();

      if object_point.y > 0.0 {
        y = y * -1.0;
      }

      object_normal = Vector::new(object_point.x, y, object_point.z).normalize();
    }

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
    ShapeType::Cone
  }
}
