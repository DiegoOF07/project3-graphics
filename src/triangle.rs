// triangle.rs
// Triangle rasterization using barycentric coordinates

use crate::Vector3;
use crate::fragment::Fragment;
use crate::light::Light;
use crate::vertex::Vertex;

/// Computes barycentric coordinates for a point in a triangle
/// Returns (w1, w2, w3) where w1 + w2 + w3 = 1 for points inside the triangle
/// Returns negative values if the point is outside the triangle
#[inline]
fn barycentric_coordinates(
    p_x: f32,
    p_y: f32,
    a: &Vertex,
    b: &Vertex,
    c: &Vertex,
) -> (f32, f32, f32) {
    let a_x = a.transformed_position.x;
    let b_x = b.transformed_position.x;
    let c_x = c.transformed_position.x;
    let a_y = a.transformed_position.y;
    let b_y = b.transformed_position.y;
    let c_y = c.transformed_position.y;

    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    // Degenerate triangle (zero area)
    if area.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    let w1 = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let w2 = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let w3 = 1.0 - w1 - w2;

    (w1, w2, w3)
}

/// Normalizes a vector in place
#[inline]
fn normalize_vector3(v: &mut Vector3) {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length > 0.0 {
        v.x /= length;
        v.y /= length;
        v.z /= length;
    }
}

/// Rasterizes a triangle and generates fragments with per-pixel lighting
/// Uses barycentric coordinates for interpolation of vertex attributes
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, light: &Light) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Base color for the surface
    let base_color = Vector3::new(0.5, 0.5, 0.5);

    // Calculate bounding box for the triangle
    let min_x = v1.transformed_position.x
        .min(v2.transformed_position.x)
        .min(v3.transformed_position.x)
        .floor() as i32;
    let max_x = v1.transformed_position.x
        .max(v2.transformed_position.x)
        .max(v3.transformed_position.x)
        .ceil() as i32;
    let min_y = v1.transformed_position.y
        .min(v2.transformed_position.y)
        .min(v3.transformed_position.y)
        .floor() as i32;
    let max_y = v1.transformed_position.y
        .max(v2.transformed_position.y)
        .max(v3.transformed_position.y)
        .ceil() as i32;

    // Iterate over bounding box and test each pixel
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p_x = x as f32 + 0.5; // Sample at pixel center
            let p_y = y as f32 + 0.5;

            let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

            // Point is inside triangle if all weights are non-negative
            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                // Interpolate normal using barycentric coordinates
                let mut interpolated_normal = Vector3::new(
                    w1 * v1.transformed_normal.x + w2 * v2.transformed_normal.x + w3 * v3.transformed_normal.x,
                    w1 * v1.transformed_normal.y + w2 * v2.transformed_normal.y + w3 * v3.transformed_normal.y,
                    w1 * v1.transformed_normal.z + w2 * v2.transformed_normal.z + w3 * v3.transformed_normal.z,
                );
                normalize_vector3(&mut interpolated_normal);

                // Interpolate world position
                let world_pos = Vector3::new(
                    w1 * v1.position.x + w2 * v2.position.x + w3 * v3.position.x,
                    w1 * v1.position.y + w2 * v2.position.y + w3 * v3.position.y,
                    w1 * v1.position.z + w2 * v2.position.z + w3 * v3.position.z,
                );

                // Calculate light direction (from surface to light)
                let mut light_dir = Vector3::new(
                    light.position.x - world_pos.x,
                    light.position.y - world_pos.y,
                    light.position.z - world_pos.z,
                );
                normalize_vector3(&mut light_dir);

                // Lambertian shading: intensity = max(0, normal · light_dir)
                let intensity = (interpolated_normal.x * light_dir.x
                    + interpolated_normal.y * light_dir.y
                    + interpolated_normal.z * light_dir.z)
                    .max(0.0);

                // Apply lighting to base color
                let shaded_color = Vector3::new(
                    base_color.x * intensity,
                    base_color.y * intensity,
                    base_color.z * intensity,
                );

                // Interpolate depth for depth testing
                let depth = w1 * v1.transformed_position.z
                    + w2 * v2.transformed_position.z
                    + w3 * v3.transformed_position.z;

                fragments.push(Fragment::new_with_world_pos(
                    p_x,
                    p_y,
                    shaded_color,
                    depth,
                    world_pos,
                ));
            }
        }
    }

    fragments
}