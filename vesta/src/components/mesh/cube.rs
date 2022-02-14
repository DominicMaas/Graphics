use cgmath::{Vector2, Vector3};

use crate::Vertex;

use super::Mesh;

impl crate::Renderer {
    /// Create a new mesh
    pub fn create_cube_mesh(&self) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut curr_index = 0;

        // BACK
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 1.0, 0.0),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 0.0, 0.0),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 1.0),
        ));

        indices.push(curr_index);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index += 4;

        // FRONT
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 0.0, 1.0),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 0.0, 1.0),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 1.0, 1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 1.0, 1.0),
            Vector2::new(0.0, 1.0),
        ));

        indices.push(curr_index);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index += 4;

        // Right
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 1.0, 1.0),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 0.0, 1.0),
            Vector2::new(0.0, 1.0),
        ));

        indices.push(curr_index);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index += 4;

        // Left
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 0.0, 0.0),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 1.0, 0.0),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 1.0, 1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 0.0, 1.0),
            Vector2::new(0.0, 1.0),
        ));

        indices.push(curr_index);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index += 4;

        // Down
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 0.0, 0.0),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 0.0, 1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 0.0, 1.0),
            Vector2::new(0.0, 1.0),
        ));

        indices.push(curr_index);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index += 4;

        // Up
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 1.0, 1.0),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(1.0, 1.0, 0.0),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(Vertex::with_tex_coords(
            Vector3::new(0.0, 1.0, 1.0),
            Vector2::new(0.0, 1.0),
        ));

        indices.push(curr_index);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        Mesh::new(vertices, indices, &self.device)
    }
}
