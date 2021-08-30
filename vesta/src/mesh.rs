use crate::cgmath::InnerSpace;
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
        // We need this for rendering
        let num_indices = indices.len() as u32;
        let num_vertices = vertices.len() as u32;

        let mut vertices_mut = vertices.to_vec();

        // Generate smooth vertices
        if num_vertices != 0 {
            for vertex in vertices_mut.iter_mut() {
                vertex.normal = cgmath::Vector3::new(0.0, 0.0, 0.0);
            }

            let mut i = 0;
            while i < indices.len() {
                let A = indices[i] as usize;
                let B = indices[i + 1] as usize;
                let C = indices[i + 2] as usize;

                let p = ((vertices_mut[B].position - vertices_mut[A].position)
                    .cross(vertices_mut[C].position - vertices_mut[A].position));

                vertices_mut[A].normal += p;
                vertices_mut[B].normal += p;
                vertices_mut[C].normal += p;

                i += 3;
            }

            for vertex in vertices_mut.iter_mut() {
                vertex.normal = vertex.normal.normalize();
            }
        }

        // Create a vertex buffer using the vertices
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices_mut.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });

        // Create an index buffer using the indices
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: wgpu::BufferUsage::INDEX,
        });

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
            self.draw(0..mesh.num_vertices, 0..1)
        } else {
            self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }
    }
}
