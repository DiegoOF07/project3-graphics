// obj.rs
// OBJ file loader for 3D models

use crate::vertex::Vertex;
use raylib::math::{Vector2, Vector3};
use tobj;

/// Represents a loaded 3D model from an OBJ file
pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Obj {
    /// Loads a 3D model from an OBJ file
    /// 
    /// # Arguments
    /// * `path` - Path to the .obj file
    /// 
    /// # Returns
    /// Result containing the loaded Obj or a LoadError
    pub fn load(path: &str) -> Result<Self, tobj::LoadError> {
        let (models, _materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // Process each vertex
            for i in 0..num_vertices {
                // Extract position (flip Y for coordinate system conversion)
                let position = Vector3::new(
                    mesh.positions[i * 3],
                    -mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2]
                );

                // Extract normal if available
                let normal = if !mesh.normals.is_empty() {
                    Vector3::new(
                        mesh.normals[i * 3],
                        mesh.normals[i * 3 + 1],
                        mesh.normals[i * 3 + 2]
                    )
                } else {
                    Vector3::zero()
                };

                // Extract texture coordinates if available
                let tex_coords = if !mesh.texcoords.is_empty() {
                    Vector2::new(
                        mesh.texcoords[i * 2],
                        mesh.texcoords[i * 2 + 1]
                    )
                } else {
                    Vector2::zero()
                };

                vertices.push(Vertex::new(position, normal, tex_coords));
            }
            
            indices.extend_from_slice(&mesh.indices);
        }

        Ok(Obj { vertices, indices })
    }

    /// Returns an indexed vertex array suitable for rendering
    /// Converts indices to actual vertex data
    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        self.indices
            .iter()
            .map(|&index| self.vertices[index as usize].clone())
            .collect()
    }
}