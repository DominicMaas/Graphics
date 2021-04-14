use cgmath::{Matrix3, Matrix4, Quaternion, Vector2, Vector3};
use vesta::DrawMesh;

pub struct Cube {
    pub mesh: vesta::Mesh,
    pub uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
}

impl Cube {
    pub fn new(renderer: &vesta::Renderer) -> Self {
        let mut vertices: Vec<vesta::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
          
        let x = -1.0;
        let y = -1.0;
        let z = -5.0;
        let mut curr_index = 0;
        
        // FRONT
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(0.0, 0.0, -1.0),
            Vector2::new(0.0, 0.0),
        ));

        indices.push(curr_index + 0);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index = curr_index + 4;
        
        // BACK
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 1.0 + z),
            Vector3::new(0.0, 0.0, 1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 1.0 + z),
            Vector3::new(0.0, 0.0, 1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 1.0 + z),
            Vector3::new(0.0, 0.0, 1.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 1.0 + z),
            Vector3::new(0.0, 0.0, 1.0),
            Vector2::new(0.0, 0.0),
        ));

        indices.push(curr_index + 0);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index = curr_index + 4;
        
        // Right
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 1.0 + z),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 1.0 + z),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));

        indices.push(curr_index + 0);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index = curr_index + 4;
        
        // Left
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 1.0 + z),
            Vector3::new(1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 1.0 + z),
            Vector3::new(1.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));

        indices.push(curr_index + 0);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index = curr_index + 4;

        // Down
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(0.0, -1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 0.0 + z),
            Vector3::new(0.0, -1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 1.0 + z),
            Vector3::new(0.0, -1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 1.0 + z),
            Vector3::new(0.0, -1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));

        indices.push(curr_index + 0);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        curr_index = curr_index + 4;
        
        // Up
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 1.0 + z),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 0.0 + z),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 1.0 + z),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));

        indices.push(curr_index + 0);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);
        
        let mesh = renderer.create_mesh(vertices, indices);

        let rotation: Quaternion<f32> = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from(rotation);
        let normal = Matrix3::from_cols(model.x.truncate(), model.y.truncate(), model.z.truncate());

        let uniform_data = vesta::ModelUniform { model, normal };
        let uniform_buffer = vesta::UniformBuffer::new(
            "Cube Uniform Buffer",
            wgpu::ShaderStage::VERTEX,
            uniform_data,
            &renderer.device,
        );
        
        Self {
            mesh,
            uniform_buffer
        }
    }
    
    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.uniform_buffer.bind_group, &[]);
        render_pass.draw_mesh(&self.mesh);
    }
}