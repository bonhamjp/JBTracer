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
pub enum ConstructiveOperation {
  Union,
  Intersection,
  Difference
}

pub struct ConstructiveGeometry<'a> {
  pub id: u64,
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub material: Material,
  pub left_side: &'a dyn Shape,
  pub right_side: &'a dyn Shape,
  pub operation: ConstructiveOperation
}

impl<'a> ConstructiveGeometry<'a> {
  pub fn new(id: u64, transform: Matrix4x4, material: Material, left_side: &'a dyn Shape, right_side: &'a dyn Shape, operation: ConstructiveOperation) -> ConstructiveGeometry<'a> {
    let tmp_inverse = transform.inverse();

    ConstructiveGeometry { 
      id: id, 
      transform: transform, 
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      material: material,
      left_side: left_side,
      right_side: right_side,
      operation: operation
    }
  }

  pub fn intersection_allowed(&self, l_hit: bool, in_l: bool, in_r: bool) -> bool {
    match self.operation {
      ConstructiveOperation::Union => {
        return (l_hit && !in_r) || (!l_hit && !in_l);
      },
      ConstructiveOperation::Intersection => {
        return (l_hit && in_r) || (!l_hit && in_l);
      },
      ConstructiveOperation::Difference => {
        return (l_hit && !in_r) || (!l_hit && in_l);
      }
    }
  }

  pub fn filter_intersections(&self, intersection_list: Vec<Intersection<'a>>) -> Vec<Intersection<'a>> {
    let mut filtered_intersection = Vec::new();

    let mut l_hit = false;
    let mut in_l = false;
    let mut in_r = false;

    // Consume intersections
    for intersection in intersection_list {
      // TODO: Implement a better way of checking object identity
      l_hit = intersection.object.is_eq(self.left_side);

      if self.intersection_allowed(l_hit, in_l, in_r) {
        // println!("HIT!!!");
        filtered_intersection.push(intersection);
      }

      // Set for next intersection allowed computation
      if l_hit {
        in_l = !in_l;
      } else {
        in_r = !in_r;
      }
    }

    filtered_intersection
  }
}

impl<'a> Shape for ConstructiveGeometry<'a> {
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

    let mut left_intersections = self.left_side.intersections(ray, self.inverse, self.transpose);
    let mut right_intersections = self.right_side.intersections(ray, self.inverse, self.transpose);

    let mut sorted_intersections = Intersection::insert_intersection(&mut left_intersections, &mut right_intersections);

    self.filter_intersections(sorted_intersections)
  }

  fn normal_at(&self, point: &Point) -> Vector {
    Vector::new(0.0, 0.0, 0.0)
  }

  fn normal_at_with_uv(&self, point: &Point, u: f64, v: f64) -> Vector {
    Vector::new(0.0, 0.0, 0.0)
  }

  fn interpolates_normals(&self) -> bool {
    false
  }

  fn get_base_type(&self) -> ShapeType {
    ShapeType::ConstructiveGeometry
  }
}
