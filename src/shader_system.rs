// shader_system.rs
// Optimized modular shader system

use raylib::prelude::*;
use crate::fragment::Fragment;
use crate::Uniforms;
use crate::noise::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ShaderType {
    Star,
    Rocky,
    GasGiant,
    Lava,
    IceWorld,
    CloudPlanet,
}

#[inline(always)]
fn mix_color(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    let t = t.clamp(0.0, 1.0);
    Vector3::new(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
    )
}

#[inline(always)]
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Star shader - Animated sun with corona and solar flares
pub fn star_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time * 0.8; // Aumentado de 0.3 para movimiento más rápido

    // Múltiples capas de ruido para más movimiento
    let surface = turbulence(Vector3::new(pos.x * 2.5, pos.y * 2.5 + time * 1.2, pos.z * 2.5), 4);
    let flares = warp_noise(Vector3::new(pos.x * 4.0 + time * 1.5, pos.y * 4.0 + time, pos.z * 4.0), 0.8);
    
    // Segundo nivel de flares para más dinamismo
    let secondary_flares = warp_noise(Vector3::new(pos.x * 2.0 + time * 0.7, pos.y * 2.0 - time * 0.5, pos.z * 2.0), 0.5);

    // Pulsos más intensos y diversos
    let pulse1 = (simplex_noise(Vector3::new(pos.x * 1.5, pos.y * 1.5, pos.z * 1.5 + time * 3.0)) * 0.5 + 0.5) * 0.8;
    let pulse2 = ((time * 1.5).sin() * 0.5 + 0.5) * 0.6; // Pulso adicional sinusoidal
    let combined_pulse = (pulse1 + pulse2) * 0.5;

    // Manchas solares que se mueven más rápido
    let spots1 = voronoi(Vector3::new(pos.x + time * 0.3, pos.y + time * 0.2, pos.z), 3.5);
    let spots2 = voronoi(Vector3::new(pos.x - time * 0.4, pos.y, pos.z + time * 0.25), 4.5);
    let spot_mask = smoothstep(0.15, 0.3, spots1) * smoothstep(0.2, 0.35, spots2);

    // Colores más vibrantes y variados
    let core_white = Vector3::new(1.0, 1.0, 1.0);
    let bright_yellow = Vector3::new(1.0, 0.95, 0.4);
    let deep_orange = Vector3::new(1.0, 0.6, 0.15);
    let hot_red = Vector3::new(1.0, 0.4, 0.2);
    let dark_spot = Vector3::new(0.7, 0.2, 0.05);

    // Mezcla de colores más dinámica
    let mut color = mix_color(bright_yellow, deep_orange, surface);
    color = mix_color(color, hot_red, flares.abs() * 0.4);
    color = mix_color(color, core_white, combined_pulse * 0.6);

    // Flares más intensos
    let flare_intensity = smoothstep(0.3, 0.75, flares.abs());
    let secondary_intensity = smoothstep(0.4, 0.8, secondary_flares.abs());
    color = mix_color(color, core_white, flare_intensity * 0.5 + secondary_intensity * 0.3);

    // Manchas solares con más contraste
    color = mix_color(color, dark_spot, (1.0 - spot_mask) * 0.85);
    
    // Brillo e iluminación intensificada
    let brightness = 1.6 + combined_pulse * 0.3; // Más brillante y con más variación
    color = color * brightness + Vector3::new(0.25, 0.15, 0.02) * combined_pulse;

    // Aplicar iluminación base
    let lit_color = color * (fragment.color * 0.4 + Vector3::new(0.6, 0.6, 0.6));
    lit_color
}

/// Rocky shader - OPTIMIZADO
pub fn rocky_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;

    // Solo 2 octavas
    let terrain = fbm(Vector3::new(pos.x * 3.0, pos.y * 3.0, pos.z * 3.0), 2, 2.0, 0.5);

    let dark_rock = Vector3::new(0.25, 0.15, 0.10);
    let mid_rock = Vector3::new(0.55, 0.35, 0.22);
    let light_rock = Vector3::new(0.85, 0.65, 0.45);

    let mut color = mix_color(dark_rock, mid_rock, terrain * 0.5 + 0.5);
    color = mix_color(color, light_rock, terrain.abs() * 0.6);

    apply_lighting(color, base_color)
}

/// Gas giant shader - OPTIMIZADO
pub fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.05;

    // Bandas simples
    let band = (pos.y * 10.0 + time).sin() * 0.5 + 0.5;
    let turb = simplex_noise(Vector3::new(pos.x * 2.0 + time, pos.y * 4.0, pos.z * 2.0));

    let light_band = Vector3::new(0.95, 0.85, 0.7);
    let mid_band = Vector3::new(0.85, 0.55, 0.35);
    let dark_band = Vector3::new(0.55, 0.35, 0.25);

    let mut color = mix_color(light_band, mid_band, band);
    color = mix_color(color, dark_band, turb.abs() * 0.3);

    apply_lighting(color, base_color)
}

/// Lava shader - OPTIMIZADO
pub fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.3;

    let flow = simplex_noise(Vector3::new(pos.x * 2.0, pos.y * 2.0 + time, pos.z * 2.0));
    let pulse = (time * 2.0).sin() * 0.2 + 0.5;

    let dark_crust = Vector3::new(0.1, 0.05, 0.0);
    let hot_lava = Vector3::new(1.0, 0.3, 0.0);
    let bright_lava = Vector3::new(1.0, 0.8, 0.1);

    let mut color = mix_color(dark_crust, hot_lava, flow * 0.5 + 0.5);
    color = mix_color(color, bright_lava, smoothstep(0.3, 0.7, flow));
    color = color + Vector3::new(pulse * 0.2, pulse * 0.05, 0.0);

    color * (base_color * 0.5 + Vector3::new(0.5, 0.5, 0.5))
}

/// Ice shader - OPTIMIZADO
pub fn ice_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;

    let snow = simplex_noise(Vector3::new(pos.x * 6.0, pos.y * 6.0, pos.z * 6.0));

    let deep_ice = Vector3::new(0.3, 0.6, 0.9);
    let surface_ice = Vector3::new(0.8, 0.9, 1.0);
    let bright_snow = Vector3::new(1.0, 1.0, 1.0);

    let mut color = mix_color(deep_ice, surface_ice, snow * 0.5 + 0.5);
    color = mix_color(color, bright_snow, smoothstep(0.5, 0.8, snow));

    let lit_color = color * (base_color + Vector3::new(0.15, 0.18, 0.22));
    normalize_intensity(lit_color, base_color, 0.15)
}

/// Cloud planet shader - OPTIMIZADO
pub fn cloud_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.1;
    
    // Reducido octavas
    let land_mask = fbm(Vector3::new(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0), 3, 2.0, 0.5);
    let clouds = simplex_noise(Vector3::new(pos.x * 4.0 + time, pos.y * 4.0, pos.z * 4.0));
    
    let ocean = Vector3::new(0.1, 0.3, 0.6);
    let land = Vector3::new(0.3, 0.45, 0.25);
    let cloud_color = Vector3::new(1.0, 1.0, 1.0);
    
    let mut color = if land_mask > 0.0 { land } else { ocean };
    
    let cloud_mask = smoothstep(0.2, 0.5, clouds);
    color = mix_color(color, cloud_color, cloud_mask * 0.6);
    
    apply_lighting(color, base_color)
}

#[inline(always)]
fn apply_lighting(color: Vector3, base_color: Vector3) -> Vector3 {
    let lit = Vector3::new(color.x * base_color.x, color.y * base_color.y, color.z * base_color.z);
    normalize_intensity(lit, base_color, 0.0)
}

#[inline(always)]
fn normalize_intensity(lit_color: Vector3, base_color: Vector3, boost: f32) -> Vector3 {
    let orig = (base_color.x + base_color.y + base_color.z) / 3.0 + boost;
    let mixed = (lit_color.x + lit_color.y + lit_color.z) / 3.0;
    if mixed > 0.001 { lit_color * (orig / mixed) } else { lit_color }
}

pub fn apply_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: ShaderType) -> Vector3 {
    match shader_type {
        ShaderType::Star => star_shader(fragment, uniforms),
        ShaderType::Rocky => rocky_shader(fragment, uniforms),
        ShaderType::GasGiant => gas_giant_shader(fragment, uniforms),
        ShaderType::Lava => lava_shader(fragment, uniforms),
        ShaderType::IceWorld => ice_shader(fragment, uniforms),
        ShaderType::CloudPlanet => cloud_planet_shader(fragment, uniforms),
    }
}