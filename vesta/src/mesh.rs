use wgpu::util::DeviceExt;

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    num_vertices: u32,
}

impl crate::Renderer {
    /// Create a new mesh
    pub fn create_mesh(&self, vertices: Vec<crate::Vertex>, indices: Vec<u32>) -> Mesh {
        Mesh::new(vertices, indices, &self.device)
    }
}

impl Mesh {
    pub fn new(vertices: Vec<crate::Vertex>, indices: Vec<u32>, device: &wgpu::Device) -> Self {
        // Create a vertex buffer using the vertices
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });

        // Create an index buffer using the indices
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: wgpu::BufferUsage::INDEX,
        });

        // We need this for rendering
        let num_indices = indices.len() as u32;
        let num_vertices = vertices.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            num_vertices,
        }
    }
}

pub trait DrawMesh<'a, 'b>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b Mesh);
}

impl<'a, 'b> DrawMesh<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b Mesh) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

        if mesh.num_indices == 0 {
            print!("Draw VERTEX");
            self.draw(0..mesh.num_vertices, 0..1)
        } else {
            print!("Draw INDEX");
            self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }
    }
}
