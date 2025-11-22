// collision.rs
// Simple collision detection for celestial objects

use raylib::prelude::*;

/// Collision system for preventing camera/ship from passing through objects
pub struct CollisionSystem;

impl CollisionSystem {
    /// Check if a position would collide with any celestial object
    /// Returns adjusted position if collision detected
    pub fn check_and_resolve(
        position: Vector3,
        objects: &[(Vector3, f32)], // (position, radius) pairs
        min_distance: f32,
    ) -> Vector3 {
        let mut result = position;
        
        for (obj_pos, radius) in objects {
            let dx = position.x - obj_pos.x;
            let dy = position.y - obj_pos.y;
            let dz = position.z - obj_pos.z;
            let dist = (dx*dx + dy*dy + dz*dz).sqrt();
            
            let safe_dist = radius + min_distance;
            
            if dist < safe_dist && dist > 0.001 {
                // Push camera out to safe distance
                let factor = safe_dist / dist;
                result.x = obj_pos.x + dx * factor;
                result.y = obj_pos.y + dy * factor;
                result.z = obj_pos.z + dz * factor;
            }
        }
        
        result
    }
    
    /// Check if moving from old_pos to new_pos would pass through any object
    pub fn check_path(
        old_pos: Vector3,
        new_pos: Vector3,
        objects: &[(Vector3, f32)],
        min_distance: f32,
    ) -> Vector3 {
        // Simple approach: check the new position
        // Could be enhanced with ray-sphere intersection for fast movement
        CollisionSystem::check_and_resolve(new_pos, objects, min_distance)
    }
}