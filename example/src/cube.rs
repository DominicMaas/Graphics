use vesta::cgmath::{Matrix3, Matrix4, Vector2, Vector3};
use vesta::DrawMesh;

pub struct Cube {
    mesh: vesta::Mesh,
    uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
    texture: vesta::Texture,
    angle: f32,
}

impl Cube {
    pub fn new(renderer: &vesta::Renderer) -> Self {
        let mut vertices: Vec<vesta::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let x = 0.0;
        let y = 0.0;
        let z = 0.0;
        let mut curr_index = 0;

        // BACK
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 0.0 + z),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 0.0 + z),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 0.0 + z),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 0.0 + z),
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
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 1.0 + z),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 1.0 + z),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 1.0 + z),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 1.0 + z),
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
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 1.0 + z),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 0.0 + z),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 0.0 + z),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 1.0 + z),
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
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 0.0 + z),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 0.0 + z),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 1.0 + z),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 1.0 + z),
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
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 0.0 + z),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 0.0 + z),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 0.0 + y, 1.0 + z),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 0.0 + y, 1.0 + z),
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
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 1.0 + z),
            Vector2::new(1.0, 1.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(1.0 + x, 1.0 + y, 0.0 + z),
            Vector2::new(1.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 0.0 + z),
            Vector2::new(0.0, 0.0),
        ));
        vertices.push(vesta::Vertex::with_tex_coords(
            Vector3::new(0.0 + x, 1.0 + y, 1.0 + z),
            Vector2::new(0.0, 1.0),
        ));

        indices.push(curr_index);
        indices.push(curr_index + 1);
        indices.push(curr_index + 3);

        indices.push(curr_index + 1);
        indices.push(curr_index + 2);
        indices.push(curr_index + 3);

        let mesh = renderer.create_mesh(vertices, indices);

        let transform = vesta::components::Transform::default();

        let uniform_data = vesta::ModelUniform {
            model: transform.calculate_model_matrix(),
            normal: transform.calculate_normal_matrix(),
        };

        let uniform_buffer = vesta::UniformBuffer::new(
            "Cube Uniform Buffer",
            vesta::wgpu::ShaderStages::VERTEX,
            uniform_data,
            &renderer.device,
        );

        let texture = renderer
            .create_texture_from_bytes(
                include_bytes!("square.png"),
                Some("square.png"),
                Default::default(),
            )
            .unwrap();

        Self {
            mesh,
            uniform_buffer,
            texture,
            angle: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, renderer: &vesta::Renderer) {
        self.angle += dt * 100.0;
        if self.angle >= 360.0 {
            self.angle = 0.0;
        }

        //let rotation = Matrix4::from_angle_z(Deg(self.angle));
        let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)); // * rotation;
        let normal = Matrix3::from_cols(model.x.truncate(), model.y.truncate(), model.z.truncate());

        self.uniform_buffer.data.model = model;
        self.uniform_buffer.data.normal = normal;
        renderer.write_uniform_buffer(&self.uniform_buffer);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut vesta::wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, self.texture.bind_group.as_ref().unwrap(), &[]);
        render_pass.set_bind_group(2, &self.uniform_buffer.bind_group, &[]);
        render_pass.draw_mesh(&self.mesh);
    }
}
