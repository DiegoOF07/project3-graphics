// vertex.rs
// Vertex data structure for 3D rendering

use raylib::math::{Vector2, Vector3};

/// Represents a vertex with all necessary attributes for rendering
#[derive(Clone, Debug)]
pub struct Vertex {
    /// Original position in model space
    pub position: Vector3,
    /// Surface normal for lighting calculations
    pub normal: Vector3,
    /// Texture coordinates (UV mapping)
    pub tex_coords: Vector2,
    /// Vertex color (if using vertex colors)
    pub color: Vector3,
    /// Position after transformation to screen space
    pub transformed_position: Vector3,
    /// Normal after transformation to world space
    pub transformed_normal: Vector3,
}

impl Vertex {
    /// Creates a new vertex with position, normal, and texture coordinates
    pub fn new(position: Vector3, normal: Vector3, tex_coords: Vector2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Vector3::new(0.0, 0.0, 0.0),
            transformed_position: position,
            transformed_normal: normal,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::new(0.0, 1.0, 0.0),
            tex_coords: Vector2::zero(),
            color: Vector3::zero(),
            transformed_position: Vector3::zero(),
            transformed_normal: Vector3::new(0.0, 1.0, 0.0),
        }
    }
}