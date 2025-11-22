// spaceship.rs
// Spaceship that follows the camera using loaded OBJ model

use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::obj::Obj;
use crate::shader_system::ShaderType;

/// Spaceship configuration and transform calculations
pub struct Spaceship {
    pub vertices: Vec<Vertex>,
    pub shader_type: ShaderType,
    pub scale: f32,
}

impl Spaceship {
    /// Load spaceship from OBJ file
    pub fn load(path: &str, scale: f32) -> Result<Self, tobj::LoadError> {
        let obj = Obj::load(path)?;
        let vertices = obj.get_vertex_array();
        
        Ok(Spaceship {
            vertices,
            shader_type: ShaderType::Spaceship,
            scale,
        })
    }
    
    /// Get the vertex array for rendering
    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    
    /// Calculate ship position and rotation based on camera
    /// Returns (position, rotation, scale)
    pub fn calculate_transform(
        camera_eye: Vector3,
        camera_target: Vector3,
        _camera_up: Vector3,
        offset_distance: f32,
        down_offset: f32,
    ) -> (Vector3, Vector3) {
        // Direction camera is looking
        let forward = Vector3::new(
            camera_target.x - camera_eye.x,
            camera_target.y - camera_eye.y,
            camera_target.z - camera_eye.z,
        );
        let len = (forward.x * forward.x + forward.y * forward.y + forward.z * forward.z).sqrt();
        
        let forward_norm = if len > 0.001 {
            Vector3::new(forward.x / len, forward.y / len, forward.z / len)
        } else {
            Vector3::new(0.0, 0.0, -1.0)
        };
        
        // Position ship in front of camera
        let position = Vector3::new(
            camera_eye.x + forward_norm.x * offset_distance,
            camera_eye.y + forward_norm.y * offset_distance - down_offset,
            camera_eye.z + forward_norm.z * offset_distance,
        );
        
        // Calculate rotation to face forward direction
        // Yaw: rotation around Y axis (horizontal direction)
        let yaw = forward_norm.z.atan2(forward_norm.x);
        
        // Pitch: rotation around X axis (vertical tilt)
        let pitch = (-forward_norm.y).asin();
        
        // Adjust rotation based on your model's default orientation
        // You may need to tweak these values depending on how your model is oriented
        let rotation = Vector3::new(
            pitch,
            yaw + std::f32::consts::FRAC_PI_2, // Adjust if model faces different direction
            0.0,
        );
        
        (position, rotation)
    }
}