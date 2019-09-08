use crate::rendering::shape::Shape;
use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

pub struct Sphere {
  pub origin: Point
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere { origin: Point::empty() }
  }
}

impl Shape for Sphere {
  fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
    let mut intersections: Vec<Intersection> = Vec::new();

    let sphere_to_ray = ray.origin.subtract_point(&self.origin);

    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    // missed if discriminant negative
    if discriminant >= 0.0 {
      intersections.push(Intersection::new((-b - discriminant.sqrt()) / (2.0 * a)));
      intersections.push(Intersection::new((-b + discriminant.sqrt()) / (2.0 * a)));
    }

    intersections
  }
}
