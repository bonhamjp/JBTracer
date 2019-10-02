use crate::rendering::shape::Shape;
use crate::rendering::shape::ShapeType;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Matrix4x4;

pub struct Triangle {
  pub id: u64,
  pub point_1: Point,
  pub point_2: Point,
  pub point_3: Point,
  pub edge_1: Vector,
  pub edge_2: Vector,
  pub normal: Vector,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub transformed_normal: Vector,
  pub material: Material
}

impl Triangle {
  pub fn new(id: u64, point_1: Point, point_2: Point, point_3: Point, transform: Matrix4x4, material: Material) -> Triangle {
    let tmp_inverse = transform.inverse();

    let edge_1 = point_2.subtract_point(&point_1);
    let edge_2 = point_3.subtract_point(&point_1);
    
    let normal = edge_2.cross(&edge_1).normalize();

    Triangle { 
      id: id,
      point_1: point_1,
      point_2: point_2,
      point_3: point_3,
      edge_1: edge_1,
      edge_2: edge_2,
      normal: normal,
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      transformed_normal: tmp_inverse.transpose().mult_vector(&normal).normalize(),
      material: material
    }
  }
}

impl Shape for Triangle {
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

    let direction_cross_e_2 = transformed_ray.direction.cross(&self.edge_2);

    let determinant = self.edge_1.dot(&direction_cross_e_2);

    // Misses if determinant is roughly zero
    if determinant.abs() < 0.0001 {
      return intersections;
    }

    let f = 1.0 / determinant;

    let p_1_to_origin = transformed_ray.origin.subtract_point(&self.point_1);

    let u = f * p_1_to_origin.dot(&direction_cross_e_2);

    // Misses if u less than zero, or above one
    if u < 0.0 || u > 1.0 {
      return intersections;
    }

    let origin_cross_e_1 = p_1_to_origin.cross(&self.edge_1);

    let v = f * transformed_ray.direction.dot(&origin_cross_e_1);

    // Misses if v is less than zero, or v + u is greater than one
    if v < 0.0 || (v + u) > 1.0 {
      return intersections;
    }

    let t = f * self.edge_2.dot(&origin_cross_e_1);

    intersections.push(Intersection::new(t, self, world_to_container, normal_to_world));

    intersections
  }

  fn normal_at(&self, point: &Point) -> Vector {
    // let mut transformed_normal = self.transpose.mult_vector(&self.normal);
    // transformed_normal.w = 0.0;
    // transformed_normal.normalize();

    // transformed_normal 

    self.transformed_normal
  }

  fn normal_at_with_uv(&self, point: &Point, u: f64, v: f64) -> Vector {
    // Not defined
    Vector::new(0.0, 0.0, 0.0)     
  }

  fn interpolates_normals(&self) -> bool {
    false
  }

  fn get_base_type(&self) -> ShapeType {
    ShapeType::Triangle
  }
}
