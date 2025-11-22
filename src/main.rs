// main.rs
// Solar System Renderer with full features

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
mod skybox;
mod spaceship;
mod orbit_renderer;
mod warp;
mod collision;

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
use shader_system::{apply_shader, ShaderType};
use solar_system::{SolarSystem, CelestialObject};
use skybox::Skybox;
use spaceship::Spaceship;
use orbit_renderer::OrbitRenderer;
use warp::WarpSystem;
use collision::CollisionSystem;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
}

fn render_object(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    light: &Light,
    shader_type: ShaderType,
) {
    let transformed: Vec<Vertex> = vertex_array
        .iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();
    
    let triangles: Vec<[Vertex; 3]> = transformed
        .chunks_exact(3)
        .map(|c| [c[0].clone(), c[1].clone(), c[2].clone()])
        .collect();
    
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }
    
    for fragment in fragments {
        let color = apply_shader(&fragment, uniforms, shader_type);
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
    framebuffer.set_background_color(Color::new(2, 2, 8, 255));
    
    // Initialize components
    let mut camera = Camera::new(
        Vector3::new(0.0, 15.0, 30.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    
    let light = Light::new(Vector3::new(0.0, 1.0, 1.0));
    
    // Load models
    let obj = Obj::load("./models/sphere.obj").expect("Failed to load sphere.obj");
    let vertex_array = obj.get_vertex_array();
    
    // Load spaceship model - adjust path and shader as needed
    let spaceship = Spaceship::load("./models/spaceship.obj", ShaderType::Rocky, 0.3)
        .expect("Failed to load spaceship.obj");
    
    // Create systems
    let mut system = SolarSystem::create_basic_system();
    let skybox = Skybox::new(WIDTH, HEIGHT, 800);
    let orbit_renderer = OrbitRenderer::new(64);
    let mut warp_system = WarpSystem::new();
    
    // Initialize warp targets
    setup_warp_targets(&mut warp_system, &system);
    
    // Display settings
    let mut show_orbits = true;
    let mut show_ship = true;
    
    let projection = create_projection_matrix(PI / 3.0, WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
    let viewport = create_viewport_matrix(0.0, 0.0, WIDTH as f32, HEIGHT as f32);

    print_controls();
    
    while !window.window_should_close() {
        let delta_time = window.get_frame_time();
        let time = window.get_time() as f32;
        
        // Handle input
        handle_system_switch(&window, &mut system, &mut warp_system);
        handle_warp_input(&window, &mut warp_system, &camera);
        handle_toggle_input(&window, &mut show_orbits, &mut show_ship);
        
        // Update warp animation
        if let Some((new_eye, new_target)) = warp_system.update(delta_time) {
            camera.set_position(new_eye, new_target);
        }
        
        // Process camera input only if not warping
        if !warp_system.is_warping() {
            camera.process_input(&window);
        }
        
        // Apply collision detection
        let collision_objects = get_collision_objects(&system);
        let safe_pos = CollisionSystem::check_and_resolve(camera.eye, &collision_objects, 0.5);
        if (safe_pos.x - camera.eye.x).abs() > 0.01 
           || (safe_pos.y - camera.eye.y).abs() > 0.01 
           || (safe_pos.z - camera.eye.z).abs() > 0.01 {
            camera.eye = safe_pos;
        }
        
        system.update(delta_time);
        framebuffer.clear();
        
        let view = camera.get_view_matrix();
        
        // Render skybox first (background)
        skybox.render(&mut framebuffer, time);
        
        // Render orbits
        if show_orbits {
            render_orbits(&orbit_renderer, &mut framebuffer, &system, &view, &projection, &viewport);
        }
        
        // Render celestial objects
        for object in &system.objects {
            let model = create_model_matrix(object.position, object.scale, object.rotation);
            let uniforms = Uniforms {
                model_matrix: model,
                view_matrix: view,
                projection_matrix: projection,
                viewport_matrix: viewport,
                time,
            };
            render_object(&mut framebuffer, &uniforms, &vertex_array, &light, object.shader_type);
        }
        
        // Render spaceship
        if show_ship {
            let (ship_pos, ship_rot) = Spaceship::calculate_transform(
                camera.eye, 
                camera.target, 
                camera.up,
                2.0,  // distance in front of camera
                0.5,  // offset below camera view
            );
            let ship_model = create_model_matrix(ship_pos, spaceship.scale, ship_rot);
            let ship_uniforms = Uniforms {
                model_matrix: ship_model,
                view_matrix: view,
                projection_matrix: projection,
                viewport_matrix: viewport,
                time,
            };
            render_object(&mut framebuffer, &ship_uniforms, spaceship.get_vertices(), &light, spaceship.shader_type);
        }
        
        framebuffer.swap_buffers(&mut window, &thread);
    }
}

fn setup_warp_targets(warp: &mut WarpSystem, system: &SolarSystem) {
    warp.warp_targets.clear();
    for (i, obj) in system.objects.iter().enumerate() {
        let name = format!("Object {}", i);
        let view_dist = obj.scale * 4.0 + 2.0;
        warp.add_target(&name, obj.position, view_dist);
    }
}

fn handle_system_switch(window: &RaylibHandle, system: &mut SolarSystem, warp: &mut WarpSystem) {
    if window.is_key_pressed(KeyboardKey::KEY_ONE) {
        *system = SolarSystem::create_basic_system();
        setup_warp_targets(warp, system);
        println!("Loaded: Basic Solar System");
    }
    if window.is_key_pressed(KeyboardKey::KEY_TWO) {
        *system = SolarSystem::create_alien_system();
        setup_warp_targets(warp, system);
        println!("Loaded: Alien Binary Star System");
    }
}

fn handle_warp_input(window: &RaylibHandle, warp: &mut WarpSystem, camera: &Camera) {
    if window.is_key_pressed(KeyboardKey::KEY_TAB) {
        warp.warp_next(camera.eye, camera.target);
        if let Some(name) = warp.current_target_name() {
            println!("Warping to: {}", name);
        }
    }
    if window.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) && window.is_key_pressed(KeyboardKey::KEY_TAB) {
        warp.warp_prev(camera.eye, camera.target);
    }
    // Number keys 3-9 for direct warp
    for i in 3..=9 {
        let key = match i {
            3 => KeyboardKey::KEY_THREE,
            4 => KeyboardKey::KEY_FOUR,
            5 => KeyboardKey::KEY_FIVE,
            6 => KeyboardKey::KEY_SIX,
            7 => KeyboardKey::KEY_SEVEN,
            8 => KeyboardKey::KEY_EIGHT,
            9 => KeyboardKey::KEY_NINE,
            _ => continue,
        };
        if window.is_key_pressed(key) {
            warp.warp_to(i - 1, camera.eye, camera.target);
        }
    }
}

fn handle_toggle_input(window: &RaylibHandle, show_orbits: &mut bool, show_ship: &mut bool) {
    if window.is_key_pressed(KeyboardKey::KEY_O) {
        *show_orbits = !*show_orbits;
        println!("Orbits: {}", if *show_orbits { "ON" } else { "OFF" });
    }
    if window.is_key_pressed(KeyboardKey::KEY_V) {
        *show_ship = !*show_ship;
        println!("Ship: {}", if *show_ship { "ON" } else { "OFF" });
    }
}

fn get_collision_objects(system: &SolarSystem) -> Vec<(Vector3, f32)> {
    system.objects.iter().map(|o| (o.position, o.scale)).collect()
}

fn render_orbits(
    renderer: &OrbitRenderer,
    fb: &mut Framebuffer,
    system: &SolarSystem,
    view: &Matrix,
    proj: &Matrix,
    vp: &Matrix,
) {
    for obj in &system.objects {
        if let Some(parent_idx) = obj.parent_index {
            let parent_pos = system.objects[parent_idx].position;
            let color = Vector3::new(0.2, 0.3, 0.5);
            renderer.render_orbit(fb, parent_pos, obj.orbit_radius, view, proj, vp, color);
        }
    }
}

fn print_controls() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║     SOLAR SYSTEM RENDERER              ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ CAMERA CONTROLS:                       ║");
    println!("║   W/S     - Pitch up/down              ║");
    println!("║   A/D     - Rotate left/right          ║");
    println!("║   Q/E     - Pan horizontally           ║");
    println!("║   R/F     - Pan vertically             ║");
    println!("║   ↑/↓     - Zoom in/out                ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ WARP CONTROLS:                         ║");
    println!("║   TAB     - Warp to next object        ║");
    println!("║   3-9     - Warp to specific object    ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ DISPLAY:                               ║");
    println!("║   O       - Toggle orbit paths         ║");
    println!("║   V       - Toggle spaceship           ║");
    println!("║   1       - Basic solar system         ║");
    println!("║   2       - Alien binary system        ║");
    println!("╠════════════════════════════════════════╣");
    println!("║   ESC     - Exit                       ║");
    println!("╚════════════════════════════════════════╝\n");
}