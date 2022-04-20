use vesta::{
    bytemuck,
    cgmath::{Matrix4, SquareMatrix, Vector4},
    UniformBuffer,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, crevice::std140::AsStd140)]
pub struct SkyUniform {
    pub proj: Matrix4<f32>,
    pub proj_inv: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub cam_pos: Vector4<f32>,
}

unsafe impl bytemuck::Zeroable for SkyUniform {}
unsafe impl bytemuck::Pod for SkyUniform {}

#[repr(C)]
#[derive(Copy, Clone, Debug, crevice::std140::AsStd140)]
pub struct SkyFragUniform {
    pub scatter_amount: f32,
}

unsafe impl bytemuck::Zeroable for SkyFragUniform {}
unsafe impl bytemuck::Pod for SkyFragUniform {}

pub struct SkyShader {
    render_pipeline: vesta::wgpu::RenderPipeline,
    pub uniform_buffer: UniformBuffer<SkyUniform>,
    pub frag_uniform_buffer: UniformBuffer<SkyFragUniform>,
}

impl SkyShader {
    pub fn new(engine: &vesta::Engine) -> Self {
        let layout =
            engine
                .renderer
                .device
                .create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
                    label: Some("Sky Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::FRAGMENT,
                            &engine.renderer.device,
                        ),
                    ],
                    push_constant_ranges: &[],
                });

        // Render pipeline for shaders
        let render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "Sky Render Pipeline",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            include_str!("res/sky_shader.wgsl").into(),
        ))
        .with_layout(&layout)
        .with_vertex_buffer_layout(&[])
        .with_depth_write_enabled(false)
        .with_depth_compare(vesta::wgpu::CompareFunction::LessEqual)
        .with_front_face(vesta::wgpu::FrontFace::Cw)
        .with_cull_mode(None)
        .build(&engine.renderer.device)
        .unwrap();

        // The uniform buffer
        let uniform_buffer = vesta::UniformBuffer::new(
            "Sky Uniform Buffer (Vertex)",
            vesta::wgpu::ShaderStages::VERTEX,
            SkyUniform {
                proj: Matrix4::identity(),
                proj_inv: Matrix4::identity(),
                view: Matrix4::identity(),
                cam_pos: Vector4::new(0.0, 0.0, 0.0, 0.0),
            },
            &engine.renderer.device,
        );

        // The uniform buffer
        let frag_uniform_buffer = vesta::UniformBuffer::new(
            "Sky Uniform Buffer (Fragment)",
            vesta::wgpu::ShaderStages::FRAGMENT,
            SkyFragUniform {
                scatter_amount: 0.0,
            },
            &engine.renderer.device,
        );

        Self {
            render_pipeline,
            uniform_buffer,
            frag_uniform_buffer,
        }
    }

    pub fn update(&self, renderer: &vesta::Renderer) {
        renderer.write_uniform_buffer(&self.uniform_buffer);
        renderer.write_uniform_buffer(&self.frag_uniform_buffer);
    }

    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        _engine: &vesta::Engine,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_buffer.bind_group, &[]);
        render_pass.set_bind_group(1, &self.frag_uniform_buffer.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
