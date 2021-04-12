#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: cgmath::Vector3<f32>,
    pub color: cgmath::Vector3<f32>,
    pub tex_coord: cgmath::Vector2<f32>,
    pub normal: cgmath::Vector3<f32>,
}

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

impl Vertex {
    /// Create a vertex with color
    #[allow(dead_code)]
    pub fn with_color(position: cgmath::Vector3<f32>, color: cgmath::Vector3<f32>) -> Self {
        Vertex {
            position,
            color,
            tex_coord: cgmath::Vector2::new(0.0, 0.0),
            normal: cgmath::Vector3::new(0.0, 0.0, 0.0),
        }
    }

    /// Create a vertex with tex coords
    pub fn with_tex_coords(
        position: cgmath::Vector3<f32>,
        normal: cgmath::Vector3<f32>,
        tex_coord: cgmath::Vector2<f32>,
    ) -> Self {
        Vertex {
            position,
            color: cgmath::Vector3::new(0.0, 0.0, 0.0),
            tex_coord,
            normal,
        }
    }

    pub(crate) fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<cgmath::Vector3<f32>>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<cgmath::Vector3<f32>>() * 2)
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<cgmath::Vector3<f32>>() * 2
                        + std::mem::size_of::<cgmath::Vector2<f32>>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}