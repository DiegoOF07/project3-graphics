// fragment.rs
// Fragment data structure for pixel-level rendering

use raylib::math::{Vector2, Vector3};

/// Represents a fragment (potential pixel) generated during rasterization
pub struct Fragment {
    /// Screen-space position (x, y coordinates)
    pub position: Vector2,
    /// Interpolated color with lighting applied
    pub color: Vector3,
    /// Depth value for depth testing
    pub depth: f32,
    /// Original position in world space (for shader calculations)
    pub world_position: Vector3,
}

impl Fragment {
    /// Creates a fragment with world position for advanced shading
    pub fn new_with_world_pos(
        x: f32,
        y: f32,
        color: Vector3,
        depth: f32,
        world_position: Vector3,
    ) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            color,
            depth,
            world_position,
        }
    }
}