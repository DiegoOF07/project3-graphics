// shader_system.rs
// Modular shader system for celestial objects

use raylib::prelude::*;
use crate::fragment::Fragment;
use crate::Uniforms;
use crate::noise::*;

/// Available shader types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ShaderType {
    Star,          // Animated sun shader
    Rocky,         // Rocky planet
    GasGiant,      // Gas giant with bands
    Lava,          // Lava planet
    IceWorld,      // Frozen planet
    CloudPlanet,   // Earth-like planet
}

/// Helper functions for color mixing
#[inline]
fn mix_color(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    let t = t.clamp(0.0, 1.0);
    Vector3::new(
        a.x * (1.0 - t) + b.x * t,
        a.y * (1.0 - t) + b.y * t,
        a.z * (1.0 - t) + b.z * t,
    )
}

#[inline]
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


/// Rocky planet shader - Mars-like with craters
pub fn rocky_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;

    // Use fast_fbm with only 3 octaves
    let terrain = fbm(Vector3::new(pos.x * 3.0, pos.y * 3.0, pos.z * 3.0), 3, 2.0, 0.5);

    // Cheaper crater pattern
    let craters = voronoi(pos, 3.5);
    let crater_mask = smoothstep(0.28, 0.48, craters);

    let dark_rock = Vector3::new(0.25, 0.15, 0.10);
    let mid_rock = Vector3::new(0.55, 0.35, 0.22);
    let light_rock = Vector3::new(0.85, 0.65, 0.45);

    let mut color = mix_color(dark_rock, mid_rock, (terrain * 0.5 + 0.5));
    // avoid powf on terrain; replace with cheaper abs or multiply
    color = mix_color(color, light_rock, (terrain.abs() * 0.8 + 0.2));
    color = mix_color(color, dark_rock * 0.9, 1.0 - crater_mask);

    apply_lighting(color, base_color)
}

/// Gas giant shader - Jupiter-like with turbulent bands
pub fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.05;

    // bands: keep sin but reduce exponent usage
    let band_pattern= (pos.y * 10.0 + time).sin() * 0.5 + 0.5;

    // cheap turbulence with fewer octaves
    let turb = turbulence(Vector3::new(pos.x * 2.0 + time, pos.y * 4.0, pos.z * 2.0), 3);

    let swirl = warp_noise(Vector3::new(pos.x + time * 0.5, pos.y * 2.0, pos.z), 0.5);

    let light_band = Vector3::new(0.95, 0.85, 0.7);
    let mid_band = Vector3::new(0.85, 0.55, 0.35);
    let dark_band = Vector3::new(0.55, 0.35, 0.25);
    let storm_color = Vector3::new(1.0, 0.9, 0.85);

    let mut color = mix_color(light_band, mid_band, band_pattern);
    color = mix_color(color, dark_band, turb * 0.35);

    let storm_mask = smoothstep(0.68, 0.82, swirl);
    color = mix_color(color, storm_color, storm_mask * 0.6);

    apply_lighting(color, base_color)
}

/// Lava planet shader - Molten world
pub fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.3;

    let lava_flow = warp_noise(Vector3::new(pos.x * 2.0, pos.y * 2.0 + time, pos.z * 2.0), 0.6);

    // cracks: use ridged_noise but fewer octaves (2 or 3)
    let cracks = ridged_noise(Vector3::new(pos.x * 5.0, pos.y * 5.0, pos.z * 5.0), 2);

    let pulse = ((time * 2.0).sin() * 0.5 + 0.5) * 0.28;

    let dark_crust = Vector3::new(0.1, 0.05, 0.0);
    let hot_lava = Vector3::new(1.0, 0.3, 0.0);
    let bright_lava = Vector3::new(1.0, 0.8, 0.1);

    let mut color = mix_color(dark_crust, hot_lava, lava_flow * 0.5 + 0.5);

    let crack_mask = smoothstep(0.62, 0.72, cracks);
    color = mix_color(color, bright_lava, crack_mask);
    color = color + Vector3::new(pulse, pulse * 0.25, 0.0);

    color * (base_color * 0.5 + Vector3::new(0.5, 0.5, 0.5))
}

/// Ice world shader - Frozen planet
pub fn ice_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.02;

    let crystals = voronoi(pos, 4.0);
    let snow = fbm(Vector3::new(pos.x * 8.0, pos.y * 8.0, pos.z * 8.0), 3, 2.0, 0.5);
    let frost = simplex_noise(Vector3::new(pos.x * 12.0 + time, pos.y * 12.0, pos.z * 12.0));

    let deep_ice = Vector3::new(0.3, 0.6, 0.9);
    let surface_ice = Vector3::new(0.8, 0.9, 1.0);
    let bright_snow = Vector3::new(1.0, 1.0, 1.0);

    let mut color = mix_color(deep_ice, surface_ice, snow * 0.45 + 0.55);
    let sparkle_mask = smoothstep(0.78, 0.88, crystals);
    color = mix_color(color, bright_snow, sparkle_mask * (frost * 0.5 + 0.5) * 1.1);

    let lit_color = color * (base_color + Vector3::new(0.18, 0.22, 0.28));
    normalize_intensity(lit_color, base_color, 0.18)
}

/// Cloud planet shader - Earth-like
pub fn cloud_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.1;
    
    // Continents vs oceans
    let land_mask = fbm_simplex(
        Vector3::new(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0),
        6, 2.0, 0.5
    );
    
    // Cloud layer
    let clouds = fbm_simplex(
        Vector3::new(pos.x * 4.0 + time, pos.y * 4.0, pos.z * 4.0),
        4, 2.0, 0.6
    );
    
    // Vegetation detail
    let vegetation = simplex_noise(
        Vector3::new(pos.x * 10.0, pos.y * 10.0, pos.z * 10.0)
    );
    
    // Colors
    let ocean = Vector3::new(0.1, 0.3, 0.6);
    let land = Vector3::new(0.4, 0.5, 0.3);
    let forest = Vector3::new(0.2, 0.4, 0.2);
    let cloud_color = Vector3::new(1.0, 1.0, 1.0);
    
    let surface_threshold = 0.0;
    let mut color = if land_mask > surface_threshold {
        mix_color(land, forest, vegetation * 0.5 + 0.5)
    } else {
        ocean
    };
    
    let cloud_mask = smoothstep(0.3, 0.5, clouds);
    color = mix_color(color, cloud_color, cloud_mask * 0.7);
    
    apply_lighting(color, base_color)
}

/// Apply standard lighting
fn apply_lighting(color: Vector3, base_color: Vector3) -> Vector3 {
    let lit_color = Vector3::new(
        color.x * base_color.x,
        color.y * base_color.y,
        color.z * base_color.z,
    );
    normalize_intensity(lit_color, base_color, 0.0)
}

/// Normalize intensity to preserve brightness
fn normalize_intensity(lit_color: Vector3, base_color: Vector3, boost: f32) -> Vector3 {
    let original_intensity = (base_color.x + base_color.y + base_color.z) / 3.0 + boost;
    let mixed_intensity = (lit_color.x + lit_color.y + lit_color.z) / 3.0;
    
    if mixed_intensity > 0.001 {
        lit_color * (original_intensity / mixed_intensity)
    } else {
        lit_color
    }
}

/// Main shader dispatcher
pub fn apply_shader(
    fragment: &Fragment,
    uniforms: &Uniforms,
    shader_type: ShaderType,
) -> Vector3 {
    match shader_type {
        ShaderType::Star => star_shader(fragment, uniforms),
        ShaderType::Rocky => rocky_shader(fragment, uniforms),
        ShaderType::GasGiant => gas_giant_shader(fragment, uniforms),
        ShaderType::Lava => lava_shader(fragment, uniforms),
        ShaderType::IceWorld => ice_shader(fragment, uniforms),
        ShaderType::CloudPlanet => cloud_planet_shader(fragment, uniforms),
    }
}