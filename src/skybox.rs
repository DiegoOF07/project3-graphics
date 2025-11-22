// skybox.rs
// Star field background rendering

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

/// Represents a star in the background
struct Star {
    x: f32,
    y: f32,
    brightness: f32,
    size: u8,
    twinkle_phase: f32,
    twinkle_speed: f32,
}

/// Skybox with procedural star field
pub struct Skybox {
    stars: Vec<Star>,
    width: i32,
    height: i32,
}

impl Skybox {
    /// Creates a new skybox with random stars
    pub fn new(width: i32, height: i32, star_count: usize) -> Self {
        let mut stars = Vec::with_capacity(star_count);
        
        for _ in 0..star_count {
            let size = if rand::random::<f32>() > 0.95 { 2 } 
                      else if rand::random::<f32>() > 0.85 { 1 } 
                      else { 0 };
            
            stars.push(Star {
                x: rand::random::<f32>() * width as f32,
                y: rand::random::<f32>() * height as f32,
                brightness: 0.3 + rand::random::<f32>() * 0.7,
                size,
                twinkle_phase: rand::random::<f32>() * std::f32::consts::PI * 2.0,
                twinkle_speed: 0.5 + rand::random::<f32>() * 2.0,
            });
        }
        
        Skybox { stars, width, height }
    }
    
    /// Renders the star field to the framebuffer
    pub fn render(&self, framebuffer: &mut Framebuffer, time: f32) {
        for star in &self.stars {
            // Twinkle effect
            let twinkle = ((time * star.twinkle_speed + star.twinkle_phase).sin() * 0.5 + 0.5) * 0.3 + 0.7;
            let brightness = star.brightness * twinkle;
            
            // Star color (slightly varied)
            let color_variation = rand::random::<f32>() * 0.1;
            let r = (brightness * (1.0 - color_variation * 0.5)).min(1.0);
            let g = (brightness * (1.0 - color_variation * 0.3)).min(1.0);
            let b = brightness.min(1.0);
            
            let color = Vector3::new(r, g, b);
            let x = star.x as i32;
            let y = star.y as i32;
            
            // Draw star (larger stars get multiple pixels)
            framebuffer.point_no_depth(x, y, color);
            
            if star.size >= 1 {
                framebuffer.point_no_depth(x + 1, y, color * 0.6);
                framebuffer.point_no_depth(x - 1, y, color * 0.6);
                framebuffer.point_no_depth(x, y + 1, color * 0.6);
                framebuffer.point_no_depth(x, y - 1, color * 0.6);
            }
            
            if star.size >= 2 {
                let dim = color * 0.3;
                framebuffer.point_no_depth(x + 1, y + 1, dim);
                framebuffer.point_no_depth(x - 1, y - 1, dim);
                framebuffer.point_no_depth(x + 1, y - 1, dim);
                framebuffer.point_no_depth(x - 1, y + 1, dim);
            }
        }
    }
}