use std::borrow::Cow;

use bytemuck::cast_slice;
use futures::executor::block_on;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Backends, BindGroupDescriptor, BindGroupEntry, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, Instance, PowerPreference,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource,
};

fn main() {
    env_logger::init();

    // wgpu instance
    let instance = Instance::new(Backends::PRIMARY);

    // Get the adapter (graphics card)
    let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .unwrap();

    // Get the device and queue used to interact with the graphics card
    let (device, queue) = block_on(adapter.request_device(&Default::default(), None)).unwrap();

    // Load in the shader
    let shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    // The pipeline that will execute this compute shader
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader,
        entry_point: "main",
    });

    // ----

    let buffer = [1u32, 2, 3];
    let size = std::mem::size_of_val(&buffer) as u64;

    let staging_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size: size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let storage_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("storage buffer"),
        contents: cast_slice(&buffer),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
    });

    // ----

    // Layout
    let bg_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bg_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch(1, 1, 1);
    }

    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Note that we're not calling `.await` here.
    let buffer_slice = staging_buffer.slice(..);
    // Gets the future representing when `staging_buffer` can be read from
    let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    if let Ok(()) = block_on(buffer_future) {
        let data = buffer_slice.get_mapped_range();
        let result = cast_slice::<u8, u32>(&data).to_vec();

        drop(data);
        staging_buffer.unmap();

        println!("{:?}", result);
    } else {
        println!("error");
    }
}
