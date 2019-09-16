use std::f64;

pub mod math;
pub mod rendering;

fn main() {
  let camera = rendering::Camera::new(
    1080, 
    720, 
    // 108,
    // 72,
    // 200,
    // 200,
    f64::consts::PI / 2.0, 
    math::Matrix4x4::view_transform(
      &math::Point::new(0.0, 0.0, -5.0), 
      &math::Point::new(0.0, 0.0, 0.0), 
      &math::Vector::new(0.0, 1.0, 0.0)
    )
  );

  let lights = vec![
    rendering::PointLight::new(
      math::Color::new(1.0, 1.0, 1.0, 1.0), 
      math::Point::new(-9.0, 8.0, -3.0)
    ),
    rendering::PointLight::new(
      math::Color::new(1.0, 1.0, 1.0, 1.0), 
      math::Point::new(4.0, 6.0, -5.0)
    )
  ];

  let sphere_1 = &rendering::Sphere::new(
    math::Matrix4x4::scale(2.0, 2.0, 2.0), 
    rendering::Material::new(math::Color::new(0.8, 0.0, 0.0, 1.0), 0.1, 0.7, 0.2, 200.0)
  );
  let plane = &rendering::Plane::new(
    math::Matrix4x4::translate(0.0, 0.0, 4.0).mult4x4(&math::Matrix4x4::rotate_x(f64::consts::PI / 2.0)), 
    rendering::Material::new(math::Color::new(1.0, 1.0, 1.0, 1.0), 0.1, 0.6, 0.6, 20.0)
  );
  let objects = vec![
    sphere_1 as &dyn rendering::shape::Shape,
    plane as &dyn rendering::shape::Shape
  ];

  let scene = rendering::Scene::new(camera, lights, objects);

  scene.render().save_image();
}
