// light.rs
// Light source for scene illumination

use raylib::prelude::*;

/// Represents a point light source in 3D space
pub struct Light {
    pub position: Vector3,
}

impl Light {
    /// Creates a new point light at the specified position
    pub fn new(position: Vector3) -> Self {
        Light { position }
    }
}