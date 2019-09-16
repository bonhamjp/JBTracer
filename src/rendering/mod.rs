pub mod canvas;
pub mod canvas_tests;
pub use self::canvas::Canvas;

pub mod scene;
pub mod scene_tests;
pub use self::scene::Scene;

pub mod camera;
pub mod camera_tests;
pub use self::camera::Camera;

pub mod point_light;
pub mod point_light_tests;
pub use self::point_light::PointLight;

pub mod shape;
pub use self::shape::Shape;

pub mod sphere;
pub mod sphere_tests;
pub use self::sphere::Sphere;

pub mod plane;
pub mod plane_tests;
pub use self::plane::Plane;

pub mod ray;
pub mod ray_tests;
pub use self::ray::Ray;

pub mod intersection;
pub mod intersection_tests;
pub use self::intersection::Intersection;

pub mod computations;
pub mod computations_tests;
pub use self::computations::Computations;

pub mod material;
pub mod material_tests;
pub use self::material::Material;
