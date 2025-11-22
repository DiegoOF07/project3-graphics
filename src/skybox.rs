// skybox.rs
// Optimized star field background - pre-computed colors

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

struct Star {
    x: i32,
    y: i32,
    base_brightness: f32,
    size: u8,
    twinkle_phase: f32,
    twinkle_speed: f32,
    // Pre-computed base color (no random en render)
    color_r: f32,
    color_g: f32,
    color_b: f32,
}

pub struct Skybox {
    stars: Vec<Star>,
}

impl Skybox {
    pub fn new(width: i32, height: i32, star_count: usize) -> Self {
        let mut stars = Vec::with_capacity(star_count);
        
        for _ in 0..star_count {
            let size = if rand::random::<f32>() > 0.95 { 2 } 
                      else if rand::random::<f32>() > 0.85 { 1 } 
                      else { 0 };
            
            // Pre-computar variación de color
            let color_var = rand::random::<f32>() * 0.15;
            let base_brightness = 0.4 + rand::random::<f32>() * 0.6;
            
            stars.push(Star {
                x: (rand::random::<f32>() * width as f32) as i32,
                y: (rand::random::<f32>() * height as f32) as i32,
                base_brightness,
                size,
                twinkle_phase: rand::random::<f32>() * std::f32::consts::PI * 2.0,
                twinkle_speed: 1.0 + rand::random::<f32>() * 1.5,
                // Colores pre-computados
                color_r: 1.0 - color_var * 0.3,
                color_g: 1.0 - color_var * 0.2,
                color_b: 1.0,
            });
        }
        
        Skybox { stars }
    }
    
    pub fn render(&self, framebuffer: &mut Framebuffer, time: f32) {
        for star in &self.stars {
            // Twinkle usando solo seno (sin random en runtime)
            let twinkle = (time * star.twinkle_speed + star.twinkle_phase).sin() * 0.2 + 0.8;
            let brightness = star.base_brightness * twinkle;
            
            let color = Vector3::new(
                (brightness * star.color_r).min(1.0),
                (brightness * star.color_g).min(1.0),
                (brightness * star.color_b).min(1.0),
            );
            
            framebuffer.point_no_depth(star.x, star.y, color);
            
            // Solo estrellas grandes tienen píxeles adicionales
            if star.size >= 1 {
                let dim = color * 0.5;
                framebuffer.point_no_depth(star.x + 1, star.y, dim);
                framebuffer.point_no_depth(star.x - 1, star.y, dim);
                framebuffer.point_no_depth(star.x, star.y + 1, dim);
                framebuffer.point_no_depth(star.x, star.y - 1, dim);
            }
            
            if star.size >= 2 {
                let vdim = color * 0.25;
                framebuffer.point_no_depth(star.x + 1, star.y + 1, vdim);
                framebuffer.point_no_depth(star.x - 1, star.y - 1, vdim);
            }
        }
    }
}