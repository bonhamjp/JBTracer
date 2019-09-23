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

pub mod pattern;
pub use self::pattern::Pattern;

pub mod solid_pattern;
pub mod solid_pattern_tests;
pub use self::solid_pattern::SolidPattern;

pub mod stripe_pattern;
pub mod stripe_pattern_tests;
pub use self::stripe_pattern::StripePattern;

pub mod gradient_pattern;
pub mod gradient_pattern_tests;
pub use self::gradient_pattern::GradientPattern;

pub mod ring_pattern;
pub mod ring_pattern_tests;
pub use self::ring_pattern::RingPattern;

pub mod checker_pattern;
pub mod checker_pattern_tests;
pub use self::checker_pattern::CheckerPattern;

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
