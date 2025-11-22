// triangle.rs
// Triangle rasterization with corrected world-space lighting

use crate::Vector3;
use crate::fragment::Fragment;
use crate::light::Light;
use crate::vertex::Vertex;
use rayon::prelude::*;

#[inline]
fn barycentric_coordinates(
    p_x: f32, p_y: f32,
    a: &Vertex, b: &Vertex, c: &Vertex,
) -> (f32, f32, f32) {
    let a_x = a.transformed_position.x;
    let b_x = b.transformed_position.x;
    let c_x = c.transformed_position.x;
    let a_y = a.transformed_position.y;
    let b_y = b.transformed_position.y;
    let c_y = c.transformed_position.y;

    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);
    if area.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    let w1 = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let w2 = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let w3 = 1.0 - w1 - w2;
    (w1, w2, w3)
}

#[inline]
fn normalize_vector3(v: &mut Vector3) {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length > 0.0 {
        v.x /= length;
        v.y /= length;
        v.z /= length;
    }
}

/// Rasteriza un triángulo con iluminación en world space
/// Ahora recibe world_positions transformadas del vertex shader
pub fn triangle(
    v1: &Vertex, 
    v2: &Vertex, 
    v3: &Vertex, 
    light: &Light,
    // Posiciones en world space (después de model matrix)
    world_pos1: Vector3,
    world_pos2: Vector3,
    world_pos3: Vector3,
) -> Vec<Fragment> {
    let base_color = Vector3::new(0.5, 0.5, 0.5);

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

    let rows: Vec<i32> = (min_y..=max_y).collect();
    
    rows.par_iter()
        .flat_map(|&y| {
            let mut row_fragments = Vec::new();
            for x in min_x..=max_x {
                let p_x = x as f32 + 0.5;
                let p_y = y as f32 + 0.5;

                let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

                if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                    // Interpolar normal transformada
                    let mut interpolated_normal = Vector3::new(
                        w1 * v1.transformed_normal.x + w2 * v2.transformed_normal.x + w3 * v3.transformed_normal.x,
                        w1 * v1.transformed_normal.y + w2 * v2.transformed_normal.y + w3 * v3.transformed_normal.y,
                        w1 * v1.transformed_normal.z + w2 * v2.transformed_normal.z + w3 * v3.transformed_normal.z,
                    );
                    normalize_vector3(&mut interpolated_normal);

                    // CORRECCIÓN: Interpolar posición en WORLD SPACE real
                    // Esto es la posición del fragmento en el espacio del mundo
                    let world_pos = Vector3::new(
                        w1 * world_pos1.x + w2 * world_pos2.x + w3 * world_pos3.x,
                        w1 * world_pos1.y + w2 * world_pos2.y + w3 * world_pos3.y,
                        w1 * world_pos1.z + w2 * world_pos2.z + w3 * world_pos3.z,
                    );

                    // Calcular dirección de luz desde el fragmento hacia la luz
                    let mut light_dir = Vector3::new(
                        light.position.x - world_pos.x,
                        light.position.y - world_pos.y,
                        light.position.z - world_pos.z,
                    );
                    normalize_vector3(&mut light_dir);

                    // Producto punto: qué tan alineada está la normal con la dirección de luz
                    let dot_product = interpolated_normal.x * light_dir.x
                        + interpolated_normal.y * light_dir.y
                        + interpolated_normal.z * light_dir.z;
                    
                    // Iluminación Lambertiana con luz ambiente
                    let ambient = 0.15; // Luz ambiente mínima para que no quede negro total
                    let diffuse = dot_product.max(0.0);
                    let intensity = ambient + (1.0 - ambient) * diffuse;

                    let shaded_color = Vector3::new(
                        base_color.x * intensity,
                        base_color.y * intensity,
                        base_color.z * intensity,
                    );

                    let depth = w1 * v1.transformed_position.z
                        + w2 * v2.transformed_position.z
                        + w3 * v3.transformed_position.z;

                    // Guardar posición en model space para shaders procedurales
                    let model_pos = Vector3::new(
                        w1 * v1.position.x + w2 * v2.position.x + w3 * v3.position.x,
                        w1 * v1.position.y + w2 * v2.position.y + w3 * v3.position.y,
                        w1 * v1.position.z + w2 * v2.position.z + w3 * v3.position.z,
                    );

                    row_fragments.push(Fragment::new_with_world_pos(
                        p_x, p_y, shaded_color, depth, model_pos
                    ));
                }
            }
            row_fragments
        })
        .collect()
}