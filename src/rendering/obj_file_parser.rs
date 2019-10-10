use std::f64;

use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader};

use crate::rendering::math::Point;
use crate::rendering::math::Vector;

use crate::rendering::math::Color;

use crate::rendering::math::Matrix4x4;

use crate::rendering::shapes::shape::Shape;
use crate::rendering::shapes::Triangle;
use crate::rendering::shapes::SmoothTriangle;

use crate::rendering::Container;

use crate::rendering::Material;

#[derive(PartialEq)]
pub enum ObjLineType {
  VertexNormal,
  Vertex,
  Face,
  Invalid
}

pub struct ObjFileParser {
  pub vertex_count: u64
}

impl ObjFileParser {
  pub fn load_file<'a>(file_name: String, transform: Matrix4x4, color: Color, triangles: &'a mut Vec<Triangle>) -> Container<'a> {
    let mut vertices = Vec::new();

    let obj_file = File::open(file_name).unwrap();
    let reader = BufReader::new(obj_file);

    for line in reader.lines() {    
      let obj_line = line.unwrap();
      
      if ObjFileParser::obj_line_type(&obj_line) == ObjLineType::Vertex {
        vertices.push(ObjFileParser::parse_vertex(&obj_line));

      } else if ObjFileParser::obj_line_type(&obj_line) == ObjLineType::Face {
        ObjFileParser::parse_face(&obj_line, color, &mut vertices, triangles);
      
      }
    }

    let mut container_shape_references = Vec::new();
    for triangle in triangles {    
      container_shape_references.push(triangle as &dyn Shape);
    }

    Container::new(transform, container_shape_references)
  }

  pub fn load_file_with_normals<'a>(file_name: String, transform: Matrix4x4, color: Color, smooth_triangles: &'a mut Vec<SmoothTriangle>) -> Container<'a> {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();

    let obj_file = File::open(file_name).unwrap();
    let reader = BufReader::new(obj_file);

    for line in reader.lines() {    
      let obj_line = line.unwrap();
      
      if ObjFileParser::obj_line_type(&obj_line) == ObjLineType::VertexNormal {
        normals.push(ObjFileParser::parse_vertex_normal(&obj_line));

      } else if ObjFileParser::obj_line_type(&obj_line) == ObjLineType::Vertex {
        vertices.push(ObjFileParser::parse_vertex(&obj_line));

      } else if ObjFileParser::obj_line_type(&obj_line) == ObjLineType::Face {
        ObjFileParser::parse_vertex_normal_face(&obj_line, color, &mut vertices, &mut normals, smooth_triangles);
      
      }
    }

    let mut container_shape_references = Vec::new();
    for smooth_triangle in smooth_triangles {    
      container_shape_references.push(smooth_triangle as &dyn Shape);
    }

    Container::new(transform, container_shape_references)
  }

  pub fn obj_line_type(obj_line: &String) -> ObjLineType {
    let chars = obj_line.chars();
    
    // Must have at least two characters
    if chars.count() < 3 {
      return ObjLineType::Invalid;
    }
    
    let bytes = obj_line.as_bytes();

    if (bytes[0] as char) == 'v' && (bytes[1] as char) == 'n' {
      return ObjLineType::VertexNormal;

    } else if (bytes[0] as char) == 'v' {
      return ObjLineType::Vertex;

    } else if (bytes[0] as char) == 'f' {
      return ObjLineType::Face;

    } else {
      return ObjLineType::Invalid;

    }
  }

  pub fn parse_vertex(obj_line: &String) -> Point {
    let vertex_line = obj_line.clone();

    let vertex_parts = vertex_line.split(" ").collect::<Vec<&str>>();
    
    // Must have exactly four parts
    Point::new(
      vertex_parts[1].parse::<f64>().unwrap(),
      vertex_parts[2].parse::<f64>().unwrap(),
      vertex_parts[3].parse::<f64>().unwrap()
    )
  }

  pub fn parse_vertex_normal(obj_line: &String) -> Vector {
    let vertex_normal_line = obj_line.clone();

    let vertex_normal_parts = vertex_normal_line.split(" ").collect::<Vec<&str>>();
    
    // Must have exactly four parts
    Vector::new(
      vertex_normal_parts[1].parse::<f64>().unwrap(),
      vertex_normal_parts[2].parse::<f64>().unwrap(),
      vertex_normal_parts[3].parse::<f64>().unwrap()
    )
  }

  pub fn parse_face(obj_line: &String, color: Color, vertices: &mut Vec::<Point>, triangles: &mut Vec::<Triangle>) {
    let face_line = obj_line.clone();

    let face_indices = face_line.split(" ").collect::<Vec<&str>>();
    
    // Split into triangles if face contains more than three vertices
    for i in 2..(face_indices.len() - 1) {
      let point_1 = vertices[face_indices[1].parse::<usize>().unwrap() - 1];
      let point_2 = vertices[face_indices[i].parse::<usize>().unwrap() - 1];
      let point_3 = vertices[face_indices[i + 1].parse::<usize>().unwrap() - 1];
      
      let triangle = Triangle::new(
        point_1,
        point_2,
        point_3,
        Matrix4x4::identity(), 
        Material::solid(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, color, Matrix4x4::identity())
      );

      triangles.push(triangle);
    }
  }

  pub fn parse_vertex_normal_face(obj_line: &String, color: Color, vertices: &mut Vec::<Point>, normals: &mut Vec::<Vector>, smooth_triangles: &mut Vec::<SmoothTriangle>) {
    let face_line = obj_line.clone();

    let face_indices = face_line.split(" ").collect::<Vec<&str>>();
    
    // Split into triangles if face contains more than three vertices
    for i in 2..(face_indices.len() - 1) {
      let index_collection_1 = face_indices[1].split("/").collect::<Vec<&str>>();
      let index_collection_2 = face_indices[i].split("/").collect::<Vec<&str>>();
      let index_collection_3 = face_indices[i + 1].split("/").collect::<Vec<&str>>();

      let point_1 = vertices[index_collection_1[0].parse::<usize>().unwrap() - 1];
      let point_2 = vertices[index_collection_2[0].parse::<usize>().unwrap() - 1];
      let point_3 = vertices[index_collection_3[0].parse::<usize>().unwrap() - 1];
      
      let normal_1 = normals[index_collection_1[2].parse::<usize>().unwrap() - 1];
      let normal_2 = normals[index_collection_2[2].parse::<usize>().unwrap() - 1];
      let normal_3 = normals[index_collection_3[2].parse::<usize>().unwrap() - 1];

      let smooth_triangle = SmoothTriangle::new(
        point_1,
        point_2,
        point_3,
        normal_1,
        normal_2,
        normal_3,
        Matrix4x4::identity(), 
        Material::solid(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, color, Matrix4x4::identity())
      );

      smooth_triangles.push(smooth_triangle);
    }
  }
}
