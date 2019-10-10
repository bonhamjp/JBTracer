use crate::rendering::math::tuple::Tuple;
use crate::rendering::math::Point;
use crate::rendering::math::Vector;

use crate::rendering::math::Matrix4x4;

use crate::rendering::shapes::shape::Shape;
use crate::rendering::shapes::shape::ShapeType;
use crate::rendering::shapes::shape::generate_shape_id;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

pub struct SmoothTriangle {
  pub id: u64,
  pub point_1: Point,
  pub point_2: Point,
  pub point_3: Point,
  pub normal_1: Vector,
  pub normal_2: Vector,
  pub normal_3: Vector,
  pub edge_1: Vector,
  pub edge_2: Vector,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub material: Material
}

impl SmoothTriangle {
  pub fn new(point_1: Point, point_2: Point, point_3: Point, normal_1: Vector, normal_2: Vector, normal_3: Vector, transform: Matrix4x4, material: Material) -> SmoothTriangle {
    let tmp_inverse = transform.inverse();

    let edge_1 = point_2.subtract_point(&point_1);
    let edge_2 = point_3.subtract_point(&point_1);
    
    SmoothTriangle {
      id: generate_shape_id(),
      point_1: point_1,
      point_2: point_2,
      point_3: point_3,
      normal_1: normal_1,
      normal_2: normal_2,
      normal_3: normal_3,
      edge_1: edge_1,
      edge_2: edge_2,
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      material: material
    }
  }
}

impl Shape for SmoothTriangle {
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

    intersections.push(Intersection::new_with_uv(t, self, world_to_container, normal_to_world, u, v));

    intersections
  }

  fn normal_at(&self, _point: &Point) -> Vector {
    // Not defined
    Vector::new(0.0, 0.0, 0.0)
  }

  fn normal_at_with_uv(&self, _point: &Point, u: f64, v: f64) -> Vector {
    // Interpolates normals
    let i_1 = self.transpose.mult_vector(&self.normal_2).multiply(u);
    let i_2 = self.transpose.mult_vector(&self.normal_3).multiply(v);
    let i_3 = self.transpose.mult_vector(&self.normal_1).multiply(1.0 - u - v);

    i_1.add_vector(&i_2).add_vector(&i_3).normalize()
  }

  fn interpolates_normals(&self) -> bool {
    true
  }

  fn get_base_type(&self) -> ShapeType {
    ShapeType::SmoothTriangle
  }
}
