// orbit_renderer.rs
// Renders orbital paths for celestial objects

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::matrix::{create_view_matrix, create_projection_matrix, create_viewport_matrix, multiply_matrix_vector4};
use std::f32::consts::PI;

/// Renders orbit paths as dotted circles
pub struct OrbitRenderer {
    segments: usize,
}

impl OrbitRenderer {
    pub fn new(segments: usize) -> Self {
        OrbitRenderer { segments }
    }
    
    /// Project a 3D point to screen space
    fn project_point(
        &self,
        point: Vector3,
        view: &Matrix,
        projection: &Matrix,
        viewport: &Matrix,
    ) -> Option<(i32, i32, f32)> {
        let p = Vector4::new(point.x, point.y, point.z, 1.0);
        
        let view_pos = multiply_matrix_vector4(view, &p);
        let clip_pos = multiply_matrix_vector4(projection, &view_pos);
        
        // Clip if behind camera
        if clip_pos.w <= 0.0 {
            return None;
        }
        
        // Perspective divide
        let ndc = Vector3::new(
            clip_pos.x / clip_pos.w,
            clip_pos.y / clip_pos.w,
            clip_pos.z / clip_pos.w,
        );
        
        // Check if in view
        if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 {
            return None;
        }
        
        let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
        let screen = multiply_matrix_vector4(viewport, &ndc_vec4);
        
        Some((screen.x as i32, screen.y as i32, screen.z))
    }
    
    /// Render an orbit circle at given center with given radius
    pub fn render_orbit(
        &self,
        framebuffer: &mut Framebuffer,
        center: Vector3,
        radius: f32,
        view: &Matrix,
        projection: &Matrix,
        viewport: &Matrix,
        color: Vector3,
    ) {
        let step = 2.0 * PI / self.segments as f32;
        
        for i in 0..self.segments {
            // Only draw every other segment for dotted effect
            if i % 2 != 0 {
                continue;
            }
            
            let angle = i as f32 * step;
            let x = center.x + radius * angle.cos();
            let z = center.z + radius * angle.sin();
            let point = Vector3::new(x, center.y, z);
            
            if let Some((sx, sy, depth)) = self.project_point(point, view, projection, viewport) {
                // Draw a small cross for visibility
                framebuffer.point_no_depth(sx, sy, color);
                framebuffer.point_no_depth(sx + 1, sy, color * 0.7);
                framebuffer.point_no_depth(sx - 1, sy, color * 0.7);
            }
        }
    }
}