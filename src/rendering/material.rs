use crate::rendering::pattern::Pattern;

use crate::math::Color;

const VACUUM_REFRACTIVE_INDEX: f64 = 1.0;
const AIR_REFRACTIVE_INDEX: f64 = 1.00029;
const WATER_REFRACTIVE_INDEX: f64 = 1.333;
const GLASS_REFRACTIVE_INDEX: f64 = 1.52;
const DIAMOND_REFRACTIVE_INDEX: f64 = 2.417;

pub struct Material<'a> {
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
  pub reflectiveness: f64,
  pub transparency: f64,
  pub refractive_index: f64,
  pub pattern: &'a dyn Pattern
}

impl<'a> Material<'a> {
  pub fn new(
    ambient: f64, 
    diffuse: f64, 
    specular: f64, 
    shininess: f64, 
    reflectiveness: f64, 
    transparency: f64, 
    refractive_index: f64, 
    pattern: &'a Pattern
  ) -> Material {
    
    Material { 
      ambient: ambient, 
      diffuse: diffuse, 
      specular: specular,
      reflectiveness: reflectiveness,
      shininess: shininess,
      transparency: transparency,
      refractive_index: refractive_index,
      pattern: pattern 
    }
  }
}
