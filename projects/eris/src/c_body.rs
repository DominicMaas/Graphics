use crate::utils::G;
use std::time::Duration;
use vesta::cgmath::{num_traits::FloatConst, Matrix3, Matrix4};
use vesta::cgmath::{Quaternion, Vector2, Vector3};

pub struct CBody {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vector3<f32>,
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub mesh: vesta::Mesh,
    pub uniform_buffer: vesta::UniformBuffer<vesta::ModelUniform>,
    pub texture: vesta::Texture,
    pub gen: CBodyGenerator,
}

impl CBody {
    pub fn new(
        name: String,
        mass: f32,
        radius: f32,
        position: Vector3<f32>,
        velocity: Vector3<f32>,
        texture: vesta::Texture,
        device: &vesta::wgpu::Device,
    ) -> Self {
        let gen = CBodyGenerator::new(radius);

        // Create the mesh for this body
        let mesh = Self::build_mesh(radius, &gen, device);
        //let mesh = Self::build_mesh_old(radius, 38, 16, device);
        let rotation: Quaternion<f32> = Quaternion::new(0.0, 0.0, 0.0, 0.0);

        let model = Matrix4::from_translation(position) * Matrix4::from(rotation);
        let normal = Self::create_normal_matrix(model);

        let uniform_data = vesta::ModelUniform { model, normal };
        let uniform_buffer = vesta::UniformBuffer::new(
            "C-Body Uniform Buffer",
            vesta::wgpu::ShaderStage::VERTEX,
            uniform_data,
            device,
        );

        Self {
            name,
            mass,
            radius,
            velocity,
            position,
            rotation,
            mesh,
            uniform_buffer,
            texture,
            gen,
        }
    }

    fn create_normal_matrix(m: Matrix4<f32>) -> Matrix3<f32> {
        Matrix3::from_cols(m.x.truncate(), m.y.truncate(), m.z.truncate())

        //let inverted = model.invert().unwrap();
        //let transposed: Matrix4<f32> = inverted.transpose();

        // Get the upper 3x3 matrix from the 4x4 matrix (upper-left)
        //Matrix3::from_cols(transposed.x.truncate(), transposed.y.truncate(), transposed.z.truncate())
    }

    pub fn standard_gravitational_parameter(&self) -> f32 {
        G * self.mass
    }

    pub fn calculate_velocity_at_radius(&self, radius: f32) -> f32 {
        (self.standard_gravitational_parameter() / radius).sqrt()
    }

    pub fn escape_velocity(&self) -> f32 {
        let n = 2.0 * self.standard_gravitational_parameter();
        let d = self.radius;
        let nd = n / d;

        nd.sqrt()
    }

    pub fn update(&mut self, _dt: Duration) {
        //let rotation_speed_deg: f32 = 0.01;
        //let rotation_speed: f32 = rotation_speed_deg * f32::PI() / 180.0;

        //let rot: Quaternion<f32> = Quaternion::from_axis_angle(
        //    Vector3::new(1.0, 0.0, 0.0).normalize(),
        //    Rad(rotation_speed),
        //);
        //self.rotation = self.rotation * rot;

        //let force = self.mass * self.velocity

        //let new_pos:  Vector3<f32> = Vector3::new(0.0, 0.01, 0.0);
        //self.position = self.position + new_pos;

        self.position += self.velocity; //* dt.as_secs_f32();
                                        //self.position = self.position + (self.velocity * _dt.as_secs_f32() * SIM_SPEED);

        // Update the uniform buffer
        let model = Matrix4::from_translation(self.position) * Matrix4::from(self.rotation);
        let normal = Self::create_normal_matrix(model);

        self.uniform_buffer.data.model = model;
        self.uniform_buffer.data.normal = normal;
    }

    fn build_mesh(radius: f32, gen: &CBodyGenerator, device: &vesta::wgpu::Device) -> vesta::Mesh {
        // Build the vertices for the mesh
        let mut vertices: Vec<vesta::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut curr_index: u32 = 0;

        let half_radius: i32 = radius as i32 / 2;

        for xi in -half_radius..half_radius {
            for yi in -half_radius..half_radius as i32 {
                for zi in -half_radius..half_radius as i32 {
                    let x = xi as f32;
                    let y = yi as f32;
                    let z = zi as f32;

                    let mat = gen.get_material(Vector3::new(x, y, z));
                    if mat == 0 {
                        continue;
                    }

                    // FRONT
                    if gen.is_transparent(x, y, z - 1.0) {
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
                    }

                    // BACK
                    if gen.is_transparent(x, y, z + 1.0) {
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
                    }

                    // Right
                    if gen.is_transparent(x - 1.0, y, z) {
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
                    }

                    // Left
                    if gen.is_transparent(x + 1.0, y, z) {
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
                    }

                    // Down
                    if gen.is_transparent(x, y - 1.0, z) {
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
                    }

                    // Up
                    if gen.is_transparent(x, y + 1.0, z) {
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

                        curr_index = curr_index + 4;
                    }
                }
            }
        }

        // Create the mesh for this body
        vesta::Mesh::new(vertices, indices, device)
    }

    fn _build_mesh_old(
        radius: f32,
        sector_count: u32,
        stack_count: u32,
        device: &vesta::wgpu::Device,
    ) -> vesta::Mesh {
        // Build the vertices for the mesh
        let mut vertices: Vec<vesta::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        // vertex position
        let mut x: f32;
        let mut y: f32;
        let mut z: f32;
        let mut xy: f32;

        // vertex normal
        let mut nx: f32;
        let mut ny: f32;
        let mut nz: f32;
        let length_inv: f32 = 1.0 / radius;

        // vertex texCoord
        let mut s: f32;
        let mut t: f32;

        let sector_step: f32 = 2.0 * f32::PI() / sector_count as f32;
        let stack_step: f32 = f32::PI() / stack_count as f32;
        let mut sector_angle: f32;
        let mut stack_angle: f32;

        for i in 0..stack_count + 1 {
            stack_angle = f32::PI() / 2.0 - i as f32 * stack_step; // starting from pi/2 to -pi/2
            xy = radius * stack_angle.cos(); // r * cos(u)
            z = radius * stack_angle.sin(); // r * sin(u)

            // add (sectorCount+1) vertices per stack
            // the first and last vertices have same position and normal, but different tex coords
            for j in 0..sector_count + 1 {
                sector_angle = j as f32 * sector_step; // starting from 0 to 2pi

                // vertex position (x, y, z)
                x = xy * sector_angle.cos(); // r * cos(u) * cos(v)
                y = xy * sector_angle.sin(); // r * cos(u) * sin(v)

                let position = Vector3::new(x, y, z);

                // normalized vertex normal (nx, ny, nz)
                nx = x * length_inv;
                ny = y * length_inv;
                nz = z * length_inv;

                let normal = Vector3::new(nx, ny, nz);

                // vertex tex coord (s, t) range between [0, 1]
                s = (j / sector_count) as f32;
                t = (i / stack_count) as f32;

                let tex_coord = Vector2::new(s, t);

                vertices.push(vesta::Vertex::with_tex_coords(position, normal, tex_coord));
            }
        }

        let mut k1: u32;
        let mut k2: u32;

        for i in 0u32..stack_count {
            k1 = i * (sector_count + 1); // beginning of current stack
            k2 = k1 + sector_count + 1; // beginning of next stack

            for _j in 0u32..sector_count {
                // 2 triangles per sector excluding first and last stacks
                // k1 => k2 => k1+1
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }

                // k1+1 => k2 => k2+1
                if i != (stack_count - 1) {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k2 + 1);
                }

                k1 += 1;
                k2 += 1;
            }
        }

        // Create the mesh for this body
        vesta::Mesh::new(vertices, indices, device)
    }
}

pub struct CBodyGenerator {
    radius: f32,
}
impl CBodyGenerator {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_transparent(&self, x: f32, y: f32, z: f32) -> bool {
        // Always show on the outside
        //if x <= 0.0 || x >= self.radius || y <= 0.0 || y >= self.radius || z <= 0.0 || z >= self.radius {
        //    return false;
        //}

        if self.get_material(Vector3::new(x, y, z)) == 0 {
            return true;
        }

        return true;
    }

    pub fn get_material(&self, position: Vector3<f32>) -> u8 {
        let half_radius: f32 = self.radius as f32 / 2.0;

        let x = half_radius + position.x;
        let y = half_radius + position.y;
        let z = half_radius + position.z;

        if ((x - self.radius / 2.0) * (x - self.radius / 2.0)
            + (y - self.radius / 2.0) * (y - self.radius / 2.0)
            + (z - self.radius / 2.0) * (z - self.radius / 2.0))
            .sqrt()
            <= self.radius / 2.0
        {
            return 1;
        }

        return 0;
    }
}
