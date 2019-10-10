use crate::rendering::math::Point;
use crate::rendering::math::Vector;

use crate::rendering::math::Matrix4x4;

use crate::rendering::shapes::shape::Shape;
use crate::rendering::shapes::shape::ShapeType;

use crate::rendering::Material;

use crate::rendering::Ray;
use crate::rendering::Intersection;

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

    let mut l_hit: bool;
    let mut in_l = false;
    let mut in_r = false;

    // Consume intersections
    for intersection in intersection_list {
      l_hit = intersection.object.is_eq(self.left_side);

      if self.intersection_allowed(l_hit, in_l, in_r) {
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

  fn intersections(&self, ray: &Ray, _world_to_container: Matrix4x4, _normal_to_world: Matrix4x4) -> Vec<Intersection> {
    let mut left_intersections = self.left_side.intersections(ray, self.inverse, self.transpose);
    let mut right_intersections = self.right_side.intersections(ray, self.inverse, self.transpose);

    let sorted_intersections = Intersection::insert_intersection(&mut left_intersections, &mut right_intersections);

    self.filter_intersections(sorted_intersections)
  }

  fn normal_at(&self, _point: &Point) -> Vector {
    Vector::new(0.0, 0.0, 0.0)
  }

  fn normal_at_with_uv(&self, _point: &Point, _u: f64, _v: f64) -> Vector {
    Vector::new(0.0, 0.0, 0.0)
  }

  fn interpolates_normals(&self) -> bool {
    false
  }

  fn get_base_type(&self) -> ShapeType {
    ShapeType::ConstructiveGeometry
  }
}
