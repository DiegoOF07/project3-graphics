// main.rs
// Main rendering loop with modular shader system

mod framebuffer;
mod triangle;
mod obj;
mod matrix;
mod fragment;
mod vertex;
mod camera;
mod shaders;
mod light;
mod noise;
mod shader_system;
mod solar_system;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::f32::consts::PI;
use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use vertex::Vertex;
use camera::Camera;
use shaders::vertex_shader;
use light::Light;
use shader_system::apply_shader;
use solar_system::{SolarSystem, CelestialObject};


/// Uniforms for shaders
pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
}

/// Render a single celestial object
fn render_object(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    light: &Light,
    object: &CelestialObject,
) {
    // Transform vertices
    let transformed: Vec<Vertex> = vertex_array
        .iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();
    
    // Assemble triangles
    let triangles: Vec<[Vertex; 3]> = transformed
        .chunks_exact(3)
        .map(|c| [c[0].clone(), c[1].clone(), c[2].clone()])
        .collect();
    
    // Rasterize
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }
    
    // Apply shader and draw
    for fragment in fragments {
        let color = apply_shader(&fragment, uniforms, object.shader_type);
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.depth,
            color,
        );
    }
}

fn main() {
    const WIDTH: i32 = 1300;
    const HEIGHT: i32 = 900;

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Solar System Renderer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Color::new(5, 5, 15, 255));
    
    // Camera setup
    let mut camera = Camera::new(
        Vector3::new(0.0, 15.0, 30.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    
    // Light source
    let light = Light::new(Vector3::new(0.0, 1.0, 1.0));
    
    // Load sphere model
    let obj = Obj::load("./models/sphere.obj")
        .expect("Failed to load sphere.obj");
    let vertex_array = obj.get_vertex_array();
    
    // Create solar system
    let mut system = SolarSystem::create_basic_system();
    let mut use_alien_system = false;
    
    // Projection matrix (constant)
    let projection = create_projection_matrix(
        PI / 3.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0
    );
    
    let viewport = create_viewport_matrix(0.0, 0.0, WIDTH as f32, HEIGHT as f32);
    
    println!("\n=== SOLAR SYSTEM RENDERER ===");
    println!("WASD - Rotate camera");
    println!("Q/E - Pan horizontally");
    println!("R/F - Pan vertically");
    println!("↑/↓ - Zoom in/out");
    println!("1 - Basic solar system");
    println!("2 - Alien binary star system");
    println!("ESC - Exit");
    println!("=============================\n");
    
    // Main loop
    while !window.window_should_close() {
        let delta_time = window.get_frame_time();
        let time = window.get_time() as f32;
        
        // Switch systems
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            system = SolarSystem::create_basic_system();
            use_alien_system = false;
            println!("Loaded: Basic Solar System");
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            system = SolarSystem::create_alien_system();
            use_alien_system = true;
            println!("Loaded: Alien Binary Star System");
        }
        
        // Update camera and system
        camera.process_input(&window);
        system.update(delta_time);
        
        // Clear buffers
        framebuffer.clear();
        
        // Get view matrix
        let view = camera.get_view_matrix();
        
        // Render all objects
        for object in &system.objects {
            let model = create_model_matrix(
                object.position,
                object.scale,
                object.rotation
            );
            
            let uniforms = Uniforms {
                model_matrix: model,
                view_matrix: view,
                projection_matrix: projection,
                viewport_matrix: viewport,
                time,
            };
            
            render_object(
                &mut framebuffer,
                &uniforms,
                &vertex_array,
                &light,
                object
            );
        }
        
        // Display
        framebuffer.swap_buffers(&mut window, &thread);
    }
}