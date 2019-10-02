#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::shape::Shape;
  use crate::rendering::Cone;
  use crate::rendering::Cube;
  use crate::rendering::Cylinder;
  use crate::rendering::Plane;
  use crate::rendering::Sphere;
  use crate::rendering::Triangle;
  use crate::rendering::SmoothTriangle;

  use crate::rendering::Container;

  use crate::rendering::Ray;
  use crate::rendering::Intersection;
  
  use crate::rendering::Material;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn container_created_with_transform() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let container = Container::new(transform, Vec::new());

    assert!(container.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
  }

  #[test]
  fn container_stores_references_to_child_shapes() {
    let transform = Matrix4x4::translate(-1.0, -1.0, -1.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(1, transform, material);

    let transform = Matrix4x4::translate(1.0, 1.0, 1.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = &Cube::new(2, transform, material);

    let transform = Matrix4x4::identity();
    let container = Container::new(transform, vec![sphere as &dyn Shape, cube as &dyn Shape]);

    assert!((container.shapes[0] as &dyn Shape).is_eq(sphere as &dyn Shape));
    assert!((container.shapes[1] as &dyn Shape).is_eq(cube as &dyn Shape));
  }



  #[test]
  fn container_stores_cones() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cone = Cone::new(1, Matrix4x4::identity(), true, -1.0, 1.0, material);

    let mut container = Container::new(Matrix4x4::identity(), Vec::new());
    container.cones = vec![cone];

    assert!(container.cones[0].get_id() == 1);
  }

  #[test]
  fn container_stores_cubes() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(1, Matrix4x4::identity(), material);

    let mut container = Container::new(Matrix4x4::identity(), Vec::new());
    container.cubes = vec![cube];

    assert!(container.cubes[0].get_id() == 1);
  }

  #[test]
  fn container_stores_cylinders() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(1, Matrix4x4::identity(), true, -1.0, 1.0, material);

    let mut container = Container::new(Matrix4x4::identity(), Vec::new());
    container.cylinders = vec![cylinder];

    assert!(container.cylinders[0].get_id() == 1);
  }

  #[test]
  fn container_stores_planes() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(1, Matrix4x4::identity(), material);

    let mut container = Container::new(Matrix4x4::identity(), Vec::new());
    container.planes = vec![plane];

    assert!(container.planes[0].get_id() == 1);
  }

  #[test]
  fn container_stores_spheres() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(1, Matrix4x4::identity(), material);

    let mut container = Container::new(Matrix4x4::identity(), Vec::new());
    container.spheres = vec![sphere];

    assert!(container.spheres[0].get_id() == 1);
  }

  #[test]
  fn container_stores_triangles() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(1, point_1, point_2, point_3, Matrix4x4::identity(), material);

    let mut container = Container::new(Matrix4x4::identity(), Vec::new());
    container.triangles = vec![triangle];

    assert!(container.triangles[0].get_id() == 1);
  }

  #[test]
  fn container_stores_smooth_triangles() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());

    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let normal_1 = Vector::new(0.0, 1.0, 0.0);
    let normal_2 = Vector::new(-1.0, 0.0, 0.0);
    let normal_3 = Vector::new(1.0, 0.0, 0.0);
    let smooth_triangle = SmoothTriangle::new(1, point_1, point_2, point_3, normal_1, normal_2, normal_3, Matrix4x4::identity(), material);

    let mut container = Container::new(Matrix4x4::identity(), Vec::new());
    container.smooth_triangles = vec![smooth_triangle];

    assert!(container.smooth_triangles[0].get_id() == 1);
  }




  #[test]
  fn intersecting_empty_container_with_a_ray() {
    let transform = Matrix4x4::identity();
    let container = Container::new(transform, Vec::new());

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = container.intersect(&ray);

    assert!(intersections.len() == 0);
  }

  #[test]
  fn intersecting_non_empty_container_at_origin() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(1, transform, material);

    let transform = Matrix4x4::translate(0.0, 0.0, -3.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(2, transform, material);

    let transform = Matrix4x4::translate(5.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_3 = &Sphere::new(3, transform, material);

    let transform = Matrix4x4::identity();
    let shapes = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape, sphere_3 as &dyn Shape];
    let container = Container::new(transform, shapes);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = container.intersect(&ray);

    assert!(intersections.len() == 4);
    assert!((intersections[0].object).is_eq(sphere_2 as &dyn Shape));
    assert!((intersections[1].object).is_eq(sphere_2 as &dyn Shape));
    assert!((intersections[2].object).is_eq(sphere_1 as &dyn Shape));
    assert!((intersections[3].object).is_eq(sphere_1 as &dyn Shape));
  }

  #[test]
  fn intersecting_non_empty_transformed_container() {
    let transform = Matrix4x4::translate(5.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(1, transform, material);

    let transform_2 = Matrix4x4::scale(2.0, 2.0, 2.0);
    let shapes = vec![sphere as &dyn Shape];
    let container = Container::new(transform_2, shapes);

    let ray = Ray::new(&Point::new(10.0, 0.0, -10.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = container.intersect(&ray);

    assert!(intersections.len() == 2);
  }
}
