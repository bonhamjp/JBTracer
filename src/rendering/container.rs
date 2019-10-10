use crate::rendering::math::Matrix4x4;

use crate::rendering::shapes::shape::Shape;

use crate::rendering::Ray;
use crate::rendering::Intersection;

pub struct Container<'a> {
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub shapes: Vec<&'a dyn Shape>
}

impl<'a> Container<'a> {
  pub fn new(transform: Matrix4x4, shapes: Vec<&'a dyn Shape> ) -> Container {
    let tmp_inverse = transform.inverse();
    
    Container { 
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      shapes: shapes
    }
  }

  pub fn intersect<'b>(&self, ray: &'b Ray) -> Vec<Intersection<'a>> {
    let transformed_point = self.inverse.mult_point(&ray.origin);
    let transformed_vector = self.inverse.mult_vector(&ray.direction);

    let transformed_ray = Ray::new(&transformed_point, &transformed_vector);
    
    let mut intersections = Vec::new();

    for shape in &self.shapes {
      let mut new_intersections = shape.intersections(&transformed_ray, self.inverse, self.transpose);

      intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    }

    intersections 
  }
}
