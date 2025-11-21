// framebuffer.rs
// Framebuffer for rendering with depth testing

use raylib::prelude::*;

/// Manages color and depth buffers for rendering
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    depth_buffer: Vec<f32>,
}

impl Framebuffer {
    /// Creates a new framebuffer with specified dimensions
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;
        let color_buffer = Image::gen_image_color(width, height, background_color);
        let depth_buffer = vec![f32::INFINITY; (width * height) as usize];
        
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            depth_buffer,
        }
    }

    /// Clears both color and depth buffers
    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
        self.depth_buffer.fill(f32::INFINITY);
    }

    /// Sets a pixel with depth testing
    /// Only draws if the new depth is closer than the existing depth
    pub fn point(&mut self, x: i32, y: i32, depth: f32, color: Vector3) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;

            // Depth test: only draw if closer to camera
            if depth < self.depth_buffer[index] {
                self.depth_buffer[index] = depth;
                
                let pixel_color = Color::new(
                    (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                );
                self.color_buffer.draw_pixel(x, y, pixel_color);
            }
        }
    }

    /// Sets the background color for clearing
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Displays the framebuffer on screen
    pub fn swap_buffers(&self, d: &mut RaylibHandle, thread: &RaylibThread) {
        if let Ok(texture) = d.load_texture_from_image(thread, &self.color_buffer) {
            let mut d = d.begin_drawing(thread);
            d.clear_background(self.background_color);
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}