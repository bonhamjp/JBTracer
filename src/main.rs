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
use rendering::shapes::Cube;
use rendering::shapes::Cylinder;
use rendering::shapes::Cone;
use rendering::shapes::Triangle;
use rendering::shapes::SmoothTriangle;

use rendering::ConstructiveGeometry;
use rendering::ConstructiveOperation;

use rendering::Container;

use rendering::Material;

use rendering::ObjFileParser;

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

  let light_1 = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(8.0, 8.0, -8.0));
  let light_2 = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-6.0, 12.0, 10.0));
  let lights = vec![light_1, light_2];

  let plane_1_transform = Matrix4x4::translate(0.0, -1.0, 0.0).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 1.0));
  let plane_1_material = Material::striped(
    0.1, 0.3, 0.3, 120.0, 0.0, 0.0, 1.0, 
    Color::new(52.0 / 255.0, 46.0 / 255.0, 55.0 / 255.0, 1.0),
    Color::new(250.0 / 255.0, 255.0 / 255.0, 253.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_1 = &Plane::new(plane_1_transform, plane_1_material);

  let plane_2_transform = Matrix4x4::translate(-40.0, 0.0, 30.0).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0).mult4x4(&Matrix4x4::rotate_z(0.6)));
  let plane_2_material = Material::solid(
    0.1, 0.3, 0.3, 120.0, 0.0, 0.0, 1.0, 
    Color::new(250.0 / 255.0, 255.0 / 255.0, 253.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_2 = &Plane::new(plane_2_transform, plane_2_material);

  let plane_3_transform = Matrix4x4::translate(40.0, 0.0, 20.0).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0).mult4x4(&Matrix4x4::rotate_z(2.6)));
  let plane_3_material = Material::solid(
    0.1, 0.3, 0.3, 120.0, 0.0, 0.0, 1.0, 
    Color::new(250.0 / 255.0, 255.0 / 255.0, 253.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_3 = &Plane::new(plane_3_transform, plane_3_material);

  let plane_4_transform = Matrix4x4::translate(0.0, 0.0, -50.0).mult4x4(&Matrix4x4::rotate_x(f64::consts::PI / 2.0));
  let plane_4_material = Material::solid(
    0.1, 0.3, 0.3, 120.0, 0.0, 0.0, 1.0, 
    Color::new(250.0 / 255.0, 255.0 / 255.0, 253.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_4 = &Plane::new(plane_4_transform, plane_4_material);

  let plane_5_transform = Matrix4x4::translate(00.0, 40.0, 0.0);
  let plane_5_material = Material::solid(
    0.1, 0.3, 0.3, 120.0, 0.0, 0.0, 1.0, 
    Color::new(250.0 / 255.0, 255.0 / 255.0, 253.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let plane_5 = &Plane::new(plane_5_transform, plane_5_material);

  let sphere_transform = Matrix4x4::translate(-3.0, 2.0, 2.0).mult4x4(&Matrix4x4::scale(2.0, 2.0, 2.0));
  let sphere_material = Material::solid(
    0.1, 0.3, 0.3, 4.0, 0.0, 0.6, 1.52, 
    Color::new(27.0 / 255.0, 147.0 / 255.0, 221.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let sphere = &Sphere::new(sphere_transform, sphere_material);

  let cylinder_transform = Matrix4x4::translate(3.0, 1.0, 1.0).mult4x4(&Matrix4x4::scale(0.9, 0.9, 0.9)).mult4x4(&Matrix4x4::rotate_x(1.2)).mult4x4(&Matrix4x4::rotate_z(0.2));
  let cylinder_material = Material::solid(
    0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, 
    Color::new(250.0 / 255.0, 130.0 / 255.0, 76.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let cylinder = &Cylinder::new(
    cylinder_transform, 
    true,
    -2.0,
    2.0,
    cylinder_material
  );

  let cone_transform = Matrix4x4::translate(0.0, 1.0, -2.0).mult4x4(&Matrix4x4::rotate_x(-0.8));
  let cone_material = Material::solid(
    0.1, 0.3, 0.3, 4.0, 1.0, 0.0, 1.0, 
    Color::new(0.0, 0.0, 0.0, 1.0),
    Matrix4x4::identity()
  );
  let cone = &Cone::new(
    cone_transform, 
    true,
    -1.0,
    0.0,
    cone_material
  );

  let cube_transform = Matrix4x4::translate(0.0, 4.0, 6.0).mult4x4(&Matrix4x4::scale(1.6, 1.6, 1.6)).mult4x4(&Matrix4x4::rotate_x(0.4)).mult4x4(&Matrix4x4::rotate_y(0.8));
  let cube_material = Material::solid(
    0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, 
    Color::new(162.0 / 255.0, 215.0 / 255.0, 41.0 / 255.0, 1.0),
    Matrix4x4::identity()
  );
  let cube_1 = &Cube::new(cube_transform, cube_material);

  let cylinder_transform = Matrix4x4::translate(0.0, 0.5, -3.0).mult4x4(&Matrix4x4::scale(0.05, 0.05, 0.05)).mult4x4(&Matrix4x4::rotate_z(-0.9)).mult4x4(&Matrix4x4::rotate_x(0.3));
  let cylinder_material = Material::solid(
    0.1, 0.3, 0.3, 4.0, 0.2 , 1.0, 1.333, 
    Color::new(0.0, 0.0, 0.0, 1.0),
    Matrix4x4::identity()
  );
  let cylinder_2 = &Cylinder::new(
    cylinder_transform, 
    true,
    -30.0,
    30.0,
    cylinder_material
  );

  // let constructive_geometry_transform = Matrix4x4::identity();
  // let constructive_geometry_material = Material::solid(
  //   0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, 
  //   Color::new(237.0 / 255.0, 174.0 / 255.0, 73.0 / 255.0, 1.0),
  //   Matrix4x4::identity()
  // );
  // let constructive_geometry = &ConstructiveGeometry::new(
  //   constructive_geometry_transform, 
  //   constructive_geometry_material,
  //   sphere,
  //   cylinder,
  //   ConstructiveOperation::Difference
  // );

  // let cube_transform = Matrix4x4::translate(-3.0, 1.0, 1.0).mult4x4(&Matrix4x4::rotate_y(0.1)).mult4x4(&Matrix4x4::rotate_z(0.9));
  // let cube_material = Material::solid(
  //   0.1, 0.3, 0.3, 4.0, 0.8, 0.0, 1.0, 
  //   Color::new(249.0 / 255.0, 223.0 / 255.0, 116.0 / 255.0, 1.0),
  //   Matrix4x4::identity()
  // );
  // let cube = &Cube::new(cube_transform, cube_material);

  let mut containers = Vec::new();

  let container_objects = vec![
    plane_1 as &dyn Shape, 
    plane_2 as &dyn Shape, 
    plane_3 as &dyn Shape, 
    plane_4 as &dyn Shape, 
    plane_5 as &dyn Shape,
    sphere as &dyn Shape,
    cylinder as &dyn Shape,
    cone as &dyn Shape,
    cube_1 as &dyn Shape,
    cylinder_2 as &dyn Shape
  ];
  let base_container = Container::new(Matrix4x4::identity(), container_objects);
  
  containers.push(base_container);

  // let obj_transform = Matrix4x4::translate(2.25, -0.75, 0.0).mult4x4(&Matrix4x4::scale(0.5, 0.5, 0.5)).mult4x4(&Matrix4x4::rotate_y(-f64::consts::PI));
  // let mut triangles: Vec<Triangle> = Vec::new();
  // let obj = ObjFileParser::load_file(
  //   "input/teapot.obj".to_string(),
  //   obj_transform, 
  //   // Color::new(249.0 / 255.0, 248.0 / 255.0, 248.0 / 255.0, 1.0),
  //   Color::new(25.0 / 255.0, 43.0 / 255.0, 194.0 / 255.0, 1.0),
  //   &mut triangles
  // );
  // containers.push(obj);

  // let obj_n_transform = Matrix4x4::translate(-2.25, 0.0, 0.0).mult4x4(&Matrix4x4::scale(0.1, 0.1, 0.1)).mult4x4(&Matrix4x4::rotate_y(f64::consts::PI / 1.2));
  // let mut smooth_triangles: Vec<SmoothTriangle> = Vec::new();
  // let obj_n = ObjFileParser::load_file_with_normals(
  //   "input/teapot_w_normals.obj".to_string(),
  //   obj_n_transform, 
  //   // Color::new(249.0 / 255.0, 248.0 / 255.0, 248.0 / 255.0, 1.0),
  //   Color::new(25.0 / 255.0, 43.0 / 255.0, 194.0 / 255.0, 1.0),
  //   &mut smooth_triangles
  // );
  // containers.push(obj_n);

  let scene = Scene::new(camera, lights, containers);

  scene.render().save_image().expect("Image file written");
}
