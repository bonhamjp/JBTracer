use crate::rendering::shape::Shape;
use crate::rendering::shape::ShapeType;
use crate::rendering::Cone;
use crate::rendering::Cube;
use crate::rendering::Cylinder;
use crate::rendering::Plane;
use crate::rendering::Sphere;
use crate::rendering::Triangle;
use crate::rendering::SmoothTriangle;

use crate::rendering::Ray;
use crate::rendering::Intersection;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Matrix4x4;

pub struct Container<'a> {
  pub transform: Matrix4x4,
  pub inverse: Matrix4x4,
  pub transpose: Matrix4x4,
  pub shapes: Vec<&'a dyn Shape>,
  
  pub cones: Vec<Cone>,
  pub cubes: Vec<Cube>,
  pub cylinders: Vec<Cylinder>,
  pub planes: Vec<Plane>,
  pub spheres: Vec<Sphere>,
  pub triangles: Vec<Triangle>,
  pub smooth_triangles: Vec<SmoothTriangle>
}

impl<'a> Container<'a> {
  pub fn new(transform: Matrix4x4, shapes: Vec<&'a dyn Shape> ) -> Container {
    let tmp_inverse = transform.inverse();
    
    Container { 
      transform: transform,
      inverse: tmp_inverse,
      transpose: tmp_inverse.transpose(),
      shapes: shapes,
      cones: Vec::new(),
      cubes: Vec::new(),
      cylinders: Vec::new(),
      planes: Vec::new(),
      spheres: Vec::new(),
      triangles: Vec::new(),
      smooth_triangles: Vec::new()
    }
  }

  pub fn intersect<'b>(&self, ray: &'b Ray) -> Vec<Intersection<'a>> {
    let transformed_point = self.inverse.mult_point(&ray.origin);
    let transformed_vector = self.inverse.mult_vector(&ray.direction);

    let transformed_ray = Ray::new(&transformed_point, &transformed_vector);
    
    // let world_to_container = self.transform.inverse();
    // let normal_to_world = world_to_container.transpose();

    // let mut intersections = Vec::new();

    // for shape in &self.shapes {
    //   let mut new_intersections = shape.intersections(&transformed_ray, world_to_container, normal_to_world);

    //   intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    // }

    let mut intersections = Vec::new();

    // Per Shape
    // TODO: Make cleaner
    for cone in &self.cones {
      let mut new_intersections = cone.intersections(&transformed_ray, self.inverse, self.transpose);
      intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    } 

    // for cube in &self.cubes {
    //   let mut new_intersections = cube.intersections(&transformed_ray, self.inverse, self.transpose);
    //   intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    // } 

    // for cylinder in &self.cylinders {
    //   let mut new_intersections = cylinder.intersections(&transformed_ray, self.inverse, self.transpose);
    //   intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    // } 

    // for plane in &self.planes {
    //   let mut new_intersections = plane.intersections(&transformed_ray, self.inverse, self.transpose);
    //   intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    // } 

    // for sphere in &self.spheres {
    //   let mut new_intersections = sphere.intersections(&transformed_ray, self.inverse, self.transpose);
    //   intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    // } 

    // for triangle in &self.triangles {
    //   let mut new_intersections = triangle.intersections(&transformed_ray, self.inverse, self.transpose);
    //   intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    // }

    // for smooth_triangle in &self.smooth_triangles {
    //   let mut new_intersections = smooth_triangle.intersections(&transformed_ray, self.inverse, self.transpose);
    //   intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    // }

    for shape in &self.shapes {
      let mut new_intersections = shape.intersections(&transformed_ray, self.inverse, self.transpose);

      intersections = Intersection::insert_intersection(&mut intersections, &mut new_intersections);
    }

    intersections 
  }
}
