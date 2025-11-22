// main.rs
// Solar System Renderer - Fixed lighting with world space positions

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
use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix, multiply_matrix_vector4};
use vertex::Vertex;
use camera::Camera;
use shaders::vertex_shader;
use light::Light;
use shader_system::{apply_shader, ShaderType};
use solar_system::SolarSystem;
use skybox::Skybox;
use spaceship::Spaceship;
use orbit_renderer::OrbitRenderer;
use warp::WarpSystem;
use collision::CollisionSystem;
use rayon::prelude::*;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
}

struct ProcessedFragment {
    x: i32,
    y: i32,
    depth: f32,
    color: Vector3,
}

/// Transforma una posición de model space a world space
#[inline]
fn transform_to_world(pos: Vector3, model_matrix: &Matrix) -> Vector3 {
    let v = Vector4::new(pos.x, pos.y, pos.z, 1.0);
    let result = multiply_matrix_vector4(model_matrix, &v);
    Vector3::new(result.x, result.y, result.z)
}

/// Renderiza un objeto con iluminación correcta en world space
fn render_object_parallel(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    light: &Light,
    shader_type: ShaderType,
) {
    // 1. Transformar vértices
    let transformed: Vec<Vertex> = vertex_array
        .par_iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();
    
    // 2. Pre-calcular posiciones en world space para cada vértice
    let world_positions: Vec<Vector3> = vertex_array
        .par_iter()
        .map(|v| transform_to_world(v.position, &uniforms.model_matrix))
        .collect();
    
    // 3. Ensamblar triángulos con sus world positions
    let triangles: Vec<([Vertex; 3], [Vector3; 3])> = transformed
        .chunks_exact(3)
        .zip(world_positions.chunks_exact(3))
        .map(|(verts, world_pos)| {
            (
                [verts[0].clone(), verts[1].clone(), verts[2].clone()],
                [world_pos[0], world_pos[1], world_pos[2]]
            )
        })
        .collect();
    
    // 4. Rasterizar con world positions correctas
    let processed: Vec<ProcessedFragment> = triangles
        .par_iter()
        .flat_map(|(tri, world_pos)| {
            let frags = triangle(
                &tri[0], &tri[1], &tri[2], 
                light,
                world_pos[0], world_pos[1], world_pos[2]
            );
            frags.into_iter().map(|frag| {
                let color = apply_shader(&frag, uniforms, shader_type);
                ProcessedFragment {
                    x: frag.position.x as i32,
                    y: frag.position.y as i32,
                    depth: frag.depth,
                    color,
                }
            }).collect::<Vec<_>>()
        })
        .collect();
    
    // 5. Escribir al framebuffer
    for frag in processed {
        framebuffer.point(frag.x, frag.y, frag.depth, frag.color);
    }
}

fn is_visible(pos: Vector3, scale: f32, camera_eye: Vector3, camera_target: Vector3) -> bool {
    let to_obj = Vector3::new(pos.x - camera_eye.x, pos.y - camera_eye.y, pos.z - camera_eye.z);
    let to_target = Vector3::new(
        camera_target.x - camera_eye.x,
        camera_target.y - camera_eye.y,
        camera_target.z - camera_eye.z,
    );
    
    let dist = (to_obj.x * to_obj.x + to_obj.y * to_obj.y + to_obj.z * to_obj.z).sqrt();
    if dist > 80.0 { return false; }
    
    let dot = to_obj.x * to_target.x + to_obj.y * to_target.y + to_obj.z * to_target.z;
    dot > -scale * 2.0 || dist < scale * 5.0
}

fn main() {
    const WIDTH: i32 = 1300;
    const HEIGHT: i32 = 900;

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap_or(());

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Solar System Renderer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Color::new(2, 2, 8, 255));
    
    let mut camera = Camera::new(
        Vector3::new(0.0, 25.0, 40.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    
    // Luz en el centro (posición del sol)
    let light = Light::new(Vector3::new(0.0, 0.0, 0.0));
    
    let obj = Obj::load("./models/sphere.obj").expect("Failed to load sphere.obj");
    let vertex_array = obj.get_vertex_array();
    
    let spaceship = Spaceship::load("./models/spaceship.obj", 0.3)
        .expect("Failed to load spaceship.obj");
    
    let mut system = SolarSystem::create_basic_system();
    let skybox = Skybox::new(WIDTH, HEIGHT, 800);
    let orbit_renderer = OrbitRenderer::new(64);
    let mut warp_system = WarpSystem::new();
    
    setup_warp_targets(&mut warp_system, &system);
    
    let mut show_orbits = true;
    let mut show_ship = true;
    
    let spaceship_position = Vector3::new(-15.0, -5.0, -20.0);
    let spaceship_rotation = Vector3::new(0.0, PI + 0.785, PI);
    
    let projection = create_projection_matrix(PI / 3.0, WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
    let viewport = create_viewport_matrix(0.0, 0.0, WIDTH as f32, HEIGHT as f32);

    print_controls();
    
    while !window.window_should_close() {
        let delta_time = window.get_frame_time();
        let time = window.get_time() as f32;
        
        handle_warp_input(&window, &mut warp_system, &camera);
        handle_toggle_input(&window, &mut show_orbits, &mut show_ship);
        
        if let Some((new_eye, new_target)) = warp_system.update(delta_time) {
            camera.set_position(new_eye, new_target);
        }
        
        if !warp_system.is_warping() {
            camera.process_input(&window);
        }
        
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
        
        skybox.render(&mut framebuffer, time);
        
        if show_orbits {
            render_orbits(&orbit_renderer, &mut framebuffer, &system, &view, &projection, &viewport);
        }
        
        // Renderizar objetos celestes
        for object in &system.objects {
            if !is_visible(object.position, object.scale, camera.eye, camera.target) {
                continue;
            }
            
            let model = create_model_matrix(object.position, object.scale, object.rotation);
            let uniforms = Uniforms {
                model_matrix: model,
                view_matrix: view,
                projection_matrix: projection,
                viewport_matrix: viewport,
                time,
            };
            render_object_parallel(&mut framebuffer, &uniforms, &vertex_array, &light, object.shader_type);
        }
        
        // Renderizar nave
        if show_ship && is_visible(spaceship_position, spaceship.scale, camera.eye, camera.target) {
            let ship_model = create_model_matrix(spaceship_position, spaceship.scale, spaceship_rotation);
            let ship_uniforms = Uniforms {
                model_matrix: ship_model,
                view_matrix: view,
                projection_matrix: projection,
                viewport_matrix: viewport,
                time,
            };
            render_object_parallel(&mut framebuffer, &ship_uniforms, spaceship.get_vertices(), &light, spaceship.shader_type);
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

fn handle_warp_input(window: &RaylibHandle, warp: &mut WarpSystem, camera: &Camera) {
    if window.is_key_pressed(KeyboardKey::KEY_TAB) {
        warp.warp_next(camera.eye, camera.target);
        if let Some(name) = warp.current_target_name() {
            println!("Warping to: {}", name);
        }
    }
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
    }
    if window.is_key_pressed(KeyboardKey::KEY_V) {
        *show_ship = !*show_ship;
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
    println!("║ WASD - Camera rotation                 ║");
    println!("║ Q/E  - Pan horizontal | R/F - Vertical ║");
    println!("║ ↑/↓  - Zoom | Z/X - Move forward/back  ║");
    println!("║ TAB  - Warp next | 3-9 - Warp to obj   ║");
    println!("║ O    - Toggle orbits | V - Toggle ship ║");
    println!("║ ESC  - Exit                            ║");
    println!("╚════════════════════════════════════════╝\n");
}