use std::f64;

pub mod rendering;

use rendering::math::Point;
use rendering::math::Vector;

use rendering::math::Color;

use rendering::math::Matrix4x4;

use rendering::Scene;

use rendering::Camera;

use rendering::PointLight;

use rendering::shapes::shape::Shape;
use rendering::shapes::Sphere;
use rendering::shapes::Plane;
use rendering::shapes::Cylinder;
use rendering::shapes::Cone;

use rendering::Container;

use rendering::Material;

fn main() {
  let camera_transform = Matrix4x4::view_transform(
    &Point::new(0.0, 0.0, -5.0), 
    &Point::new(0.0, 0.0, 0.0), 
    &Vector::new(0.0, 1.0, 0.0)
  );
  let camera = Camera::new(
    200,
    200,
    f64::consts::PI / 2.0, 
    camera_transform
  );

  let mut containers = Vec::new();

  let light_1 = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-8.0, 4.0, -2.0));
  let light_2 = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(10.0, 10.0, 6.0));
  let lights = vec![light_1, light_2];

  let plane_1_transform = Matrix4x4::translate(0.0, -2.0, 0.0).mult4x4(&Matrix4x4::rotate_x(0.1));
  let plane_1_material = Material::solid(
    0.1, 0.3, 0.3, 10.0, 0.1, 0.0, 1.0, 
    Color::new(64.0 / 255.0, 67.0 / 255.0, 78.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_1 = &Plane::new(plane_1_transform, plane_1_material);

  let plane_2_transform = Matrix4x4::translate(0.0, 0.0, 20.0).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0));
  let plane_2_material = Material::solid(
    0.1, 0.3, 0.3, 10.0, 0.0, 0.0, 1.0, 
    Color::new(255.0 / 255.0, 255.0 / 255.0, 250.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_2 = &Plane::new(plane_2_transform, plane_2_material);

  let sphere_transform = Matrix4x4::translate(-2.0, 1.5, 1.0);
  let sphere_material = Material::solid(
    0.1, 0.3, 0.3, 100.0, 0.25, 0.0, 1.0, 
    Color::new(145.0 / 255.0, 47.0 / 255.0, 64.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let sphere = &Sphere::new(sphere_transform, sphere_material);

  let cone_transform = Matrix4x4::translate(-2.0, 0.0, 1.0);
  let cone_material = Material::solid(
    0.1, 0.3, 0.3, 100.0, 0.25, 0.0, 1.0, 
    Color::new(145.0 / 255.0, 47.0 / 255.0, 64.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let cone = &Cone::new(
    cone_transform, 
    true,
    -1.0,
    0.0,
    cone_material
  );

  let cylinder_transform = Matrix4x4::translate(2.0, 1.0, 1.0);
  let cylinder_material = Material::solid(
    0.1, 0.3, 0.3, 100.0, 0.25, 0.0, 1.0, 
    Color::new(112.0 / 255.0, 38.0 / 255.0, 50.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let cylinder = &Cylinder::new(
    cylinder_transform, 
    true,
    -2.0,
    2.0,
    cylinder_material
  );

  let container_objects = vec![
    plane_1 as &dyn Shape, 
    plane_2 as &dyn Shape,
    sphere as &dyn Shape,
    cone as &dyn Shape,
    cylinder as &dyn Shape
  ];
  let base_container = Container::new(Matrix4x4::identity(), container_objects);

  containers.push(base_container);

  let scene = Scene::new(camera, lights, containers);

  scene.render().save_image().expect("Image file written");
}
