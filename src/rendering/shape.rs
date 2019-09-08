use crate::rendering::Ray;
use crate::rendering::Intersection;

pub trait Shape {
  fn intersections(&self, ray: &Ray) -> Vec<Intersection>;
}
