use crate::rendering::shape::Shape;
use crate::rendering::shape::ShapeType;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Matrix4x4;

pub struct Cube {
  pub id: u64,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub material: Material
}

impl Cube {
  pub fn new(id: u64, transform: Matrix4x4, material: Material) -> Cube {
    let tmp_inverse = transform.inverse();

    Cube { 
      id: id, 
      transform: transform, 
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      material: material
    }
  }

  pub fn get_axis_bounds(&self, origin_s: f64, direction_s: f64) -> (f64, f64) {
    let mut t_min;
    let mut t_max;
    
    let t_min_numerator = (-1.0 - origin_s);
    let t_max_numerator = (1.0 - origin_s);

    if (direction_s).abs() >= 0.0001 { // TODO: Use global epsilon 
      t_min = t_min_numerator / direction_s;
      t_max = t_max_numerator / direction_s;
    } else {
      t_min = t_min_numerator * 9999999.9;
      t_max = t_max_numerator * 9999999.9;
    }

    if t_min > t_max {
      let tmp_t = t_min;
      
      t_min = t_max;
      t_max = tmp_t;
    }

    (t_min, t_max)
  }
}

impl Shape for Cube {
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

    let (x_min, x_max) = self.get_axis_bounds(transformed_ray.origin.x, transformed_ray.direction.x);
    let (y_min, y_max) = self.get_axis_bounds(transformed_ray.origin.y, transformed_ray.direction.y);
    let (z_min, z_max) = self.get_axis_bounds(transformed_ray.origin.z, transformed_ray.direction.z);

    let t_min;
    let t_max;

    // t_min is maximum of axis mins
    if x_min > y_min && x_min > z_min {
      t_min = x_min;
    } else if y_min > x_min && y_min > z_min {
      t_min = y_min;
    } else {
      t_min = z_min;
    }

    // t_max is the minimum of axis maxs
    if x_max <= y_max && x_max <= z_max {
      t_max = x_max;
    } else if y_max <= x_max && y_max <= z_max {
      t_max = y_max;
    } else {
      t_max = z_max;
    }

    // No intersections if t_min is larger than t_max
    if t_min <= t_max {
      intersections.push(Intersection::new(t_min, self, world_to_container, normal_to_world));
      intersections.push(Intersection::new(t_max, self, world_to_container, normal_to_world));    
    }

    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    let object_point = self.inverse.mult_point(point);
    
    let x_abs = object_point.x.abs();
    let y_abs = object_point.y.abs();
    let z_abs = object_point.z.abs();

    let object_normal;

    // Normal is pointing in direction of axis hit
    if x_abs >= y_abs && x_abs >= z_abs {
      // return Vector::new(object_point.x, 0.0, 0.0).normalize();
      object_normal = Vector::new(object_point.x, 0.0, 0.0);
    } else if y_abs >= x_abs && y_abs >= z_abs {
      // return Vector::new(0.0, object_point.y, 0.0).normalize();
      object_normal = Vector::new(0.0, object_point.y, 0.0);
    } else {
      // return Vector::new(0.0, 0.0, object_point.z).normalize();
      object_normal = Vector::new(0.0, 0.0, object_point.z);
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
    ShapeType::Cube
  }
}
