pub mod tuple;

pub mod vector;
pub mod vector_tests;
pub use self::vector::Vector;

pub mod point;
pub mod point_tests;
pub use self::point::Point;

pub mod color;
pub mod color_tests;
pub use self::color::Color;

pub mod matrix2x2;
pub mod matrix2x2_tests;
pub use self::matrix2x2::Matrix2x2;

pub mod matrix3x3;
pub mod matrix3x3_tests;
pub use self::matrix3x3::Matrix3x3;

pub mod matrix4x4;
pub mod matrix4x4_tests;
pub use self::matrix4x4::Matrix4x4;
