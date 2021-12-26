use bytemuck::Pod;
use bytemuck::Zeroable;
use cgmath::Vector3;
use cgmath::Vector4;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;

use crevice::std140::AsStd140;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsStd140)]
pub struct ModelUniform {
    pub model: cgmath::Matrix4<f32>,  // 4x4 matrix
    pub normal: cgmath::Matrix3<f32>, // 3x3 matrix
}

unsafe impl Zeroable for ModelUniform {}
unsafe impl Pod for ModelUniform {}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsStd140)]
pub struct LightUniform {
    pub position: Vector4<f32>,
    pub color: Vector4<f32>,
}

unsafe impl Zeroable for LightUniform {}
unsafe impl Pod for LightUniform {}

impl LightUniform {
    pub fn new(position: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self {
            position: Vector4::new(position.x, position.y, position.z, 1.0),
            color: Vector4::new(color.x, color.y, color.z, 1.0),
        }
    }
}

// ----------------------------------------- //

/// A holder for a uniform buffer, contains the data and raw buffer
pub struct UniformBuffer<T>
where
    T: Copy + bytemuck::Pod + bytemuck::Zeroable + AsStd140,
{
    pub data: T,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl crate::Renderer {
    /// Write the specified uniform buffer to the GPU
    pub fn write_uniform_buffer<T: Copy + bytemuck::Pod + bytemuck::Zeroable + AsStd140>(
        &self,
        uniform_buffer: &UniformBuffer<T>,
    ) {
        //engine.renderer.queue.write_buffer(
        //    &body.uniform_buffer.buffer,
        //    0,
        //    vesta::bytemuck::cast_slice(&[body.uniform_buffer.data]),
        //);

        self.queue.write_buffer(
            &uniform_buffer.buffer,
            0,
            bytemuck::cast_slice(&[uniform_buffer.data.as_std140()]),
        );
    }
}

impl<T: Copy + bytemuck::Pod + bytemuck::Zeroable + AsStd140> UniformBuffer<T> {
    #[deprecated(note = "Please use renderer.write_uniform_buffer() instead")]
    pub fn write_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.data]));
    }

    //noinspection RsBorrowChecker
    /// Crate a new uniform buffer to store data of type
    pub fn new(name: &str, visibility: wgpu::ShaderStages, data: T, device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(name),
            contents: bytemuck::cast_slice(&[data.as_std140()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &UniformBufferUtils::create_bind_group_layout(visibility, device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        Self {
            data,
            buffer,
            bind_group,
        }
    }
}

pub struct UniformBufferUtils {}
impl UniformBufferUtils {
    pub fn create_bind_group_layout(
        visibility: wgpu::ShaderStages,
        device: &wgpu::Device,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        })
    }
}
