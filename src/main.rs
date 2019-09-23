use std::f64;

pub mod math;

// TODO: Use preamble

use math::Point;
use math::Vector;

use math::Color;

use math::Matrix4x4;

pub mod rendering;

// TODO: Use preamble

use rendering::Scene;

use rendering::Camera;

use rendering::PointLight;

use rendering::shape::Shape;
use rendering::Sphere;
use rendering::Plane;

use rendering::Material;

use rendering::SolidPattern;
use rendering::StripePattern;
use rendering::GradientPattern;
use rendering::RingPattern;
use rendering::CheckerPattern;

fn main() {
  let camera_transform = Matrix4x4::view_transform(
    &Point::new(0.0, 0.0, -5.0), 
    &Point::new(0.0, 0.0, 0.0), 
    &Vector::new(0.0, 1.0, 0.0)
  );
  let camera = Camera::new(
    // 1080, 
    // 720, 
    108,
    72,
    f64::consts::PI / 2.0, 
    camera_transform
  );

  let light_1 = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(6.0, 3.0, 0.0));
  let light_2 = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-4.0, 8.0, -8.0));
  let lights = vec![light_1, light_2];

  let sphere_1_transform = Matrix4x4::scale(1.5, 1.5, 1.5);
  let sphere_1_pattern = &SolidPattern::new(
    // Color::new(255.0 / 255.0, 66.0 / 255.0, 66.0 / 255.0, 1.0),
    Color::new(1.0, 1.0, 1.0, 1.0),
    Matrix4x4::identity()
  );
  let sphere_1_material = Material::new(0.1, 0.5, 0.4, 20.0, 0.25, 0.75, 1.5, sphere_1_pattern);
  let sphere_1 = &Sphere::new(1, sphere_1_transform, sphere_1_material);

  let sphere_2_transform = Matrix4x4::scale(0.5, 0.5, 0.5);
  let sphere_2_pattern = &SolidPattern::new(
    Color::new(251.0 / 255.0, 98.0 / 255.0, 246.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let sphere_2_material = Material::new(0.1, 0.5, 0.4, 20.0, 0.75, 0.25, 1.5, sphere_2_pattern);
  let sphere_2 = &Sphere::new(8, sphere_2_transform, sphere_2_material);

  let plane_1_transform = Matrix4x4::translate(0.0, -4.0, 0.0);
  let plane_1_pattern = &CheckerPattern::new(
    Color::new(100.0 / 255.0, 93.0 / 255.0, 215.0 / 255.0, 1.0),
    Color::new(1.0, 1.0, 1.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_1_material = Material::new(0.1, 0.5, 0.5, 10.0, 0.25, 0.0, 1.0, plane_1_pattern);
  let plane_1 = &Plane::new(2, plane_1_transform, plane_1_material);

  let plane_2_transform = Matrix4x4::translate(0.0, 0.0, 16.0).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0));
  let plane_2_pattern = &SolidPattern::new(
    Color::new(100.0 / 255.0, 93.0 / 255.0, 215.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_2_material = Material::new(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, plane_2_pattern);
  let plane_2 = &Plane::new(3, plane_2_transform, plane_2_material);

  let plane_3_transform = Matrix4x4::translate(0.0, 0.0, -10.0).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0));
  let plane_3_pattern = &SolidPattern::new(
    Color::new(100.0 / 255.0, 93.0 / 255.0, 215.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_3_material = Material::new(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, plane_3_pattern);
  let plane_3 = &Plane::new(4, plane_3_transform, plane_3_material);

  let plane_4_transform = Matrix4x4::translate(10.0, 0.0, 0.0).mult4x4(&Matrix4x4::rotate_y(f64::consts::PI / 2.0)).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0));
  let plane_4_pattern = &SolidPattern::new(
    Color::new(100.0 / 255.0, 93.0 / 255.0, 215.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_4_material = Material::new(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, plane_4_pattern);
  let plane_4 = &Plane::new(5, plane_4_transform, plane_4_material);

  let plane_5_transform = Matrix4x4::translate(-10.0, 0.0, 0.0).mult4x4(&Matrix4x4::rotate_y(f64::consts::PI / 2.0)).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0));
  let plane_5_pattern = &SolidPattern::new(
    Color::new(100.0 / 255.0, 93.0 / 255.0, 215.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_5_material = Material::new(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, plane_5_pattern);
  let plane_5 = &Plane::new(6, plane_5_transform, plane_5_material);

  let plane_6_transform = Matrix4x4::translate(0.0, 16.0, 0.0);
  let plane_6_pattern = &SolidPattern::new(
    Color::new(100.0 / 255.0, 93.0 / 255.0, 215.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_6_material = Material::new(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, plane_6_pattern);
  let plane_6 = &Plane::new(7, plane_6_transform, plane_6_material);

  let objects = vec![
    sphere_1 as &dyn Shape,
    sphere_2 as &dyn Shape,
    plane_1 as &dyn Shape,
    plane_2 as &dyn Shape,
    plane_3 as &dyn Shape,
    plane_4 as &dyn Shape,
    plane_5 as &dyn Shape,
    plane_6 as &dyn Shape
  ];

  let scene = Scene::new(camera, lights, objects);

  scene.render().save_image();
}
