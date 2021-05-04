use anyhow::*;

pub struct RenderPipelineBuilder<'a> {
    layout: Option<&'a wgpu::PipelineLayout>,
    shader_source: Option<wgpu::ShaderModuleDescriptor<'a>>,
    vertex_shader_entry: &'a str,
    fragment_shader_entry: &'a str,
    texture_format: wgpu::TextureFormat,
    pipeline_name: &'a str,
    primitive_topology: wgpu::PrimitiveTopology,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(
        texture_format: wgpu::TextureFormat,
        pipeline_name: &'a str,
    ) -> RenderPipelineBuilder {
        Self {
            layout: None,
            shader_source: None,
            vertex_shader_entry: "vs_main",
            fragment_shader_entry: "fs_main",
            texture_format,
            pipeline_name,
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        }
    }

    pub fn with_layout(&mut self, layout: &'a wgpu::PipelineLayout) -> &mut Self {
        self.layout = Some(layout);
        self
    }

    pub fn with_shader_source(&mut self, source: wgpu::ShaderSource<'a>) -> &mut Self {
        self.shader_source = Some(wgpu::ShaderModuleDescriptor {
            label: None,
            source,
            flags: wgpu::ShaderFlags::VALIDATION | wgpu::ShaderFlags::EXPERIMENTAL_TRANSLATION,
        });
        self
    }

    pub fn with_vertex_shader_entry(&mut self, vertex_shader_entry: &'a str) -> &mut Self {
        self.vertex_shader_entry = vertex_shader_entry;
        self
    }

    pub fn with_fragment_shader_entry(&mut self, fragment_shader_entry: &'a str) -> &mut Self {
        self.fragment_shader_entry = fragment_shader_entry;
        self
    }

    #[allow(dead_code)]
    pub fn with_topology(&mut self, topology: wgpu::PrimitiveTopology) -> &mut Self {
        self.primitive_topology = topology;
        self
    }

    pub fn build(&mut self, device: &wgpu::Device) -> Result<wgpu::RenderPipeline> {
        // Ensure layout
        if self.layout.is_none() {
            bail!("No pipeline layout was supplied!");
        }
        let layout = self.layout.unwrap();

        // Ensure shader source
        if self.shader_source.is_none() {
            bail!("No shader source supplied!");
        }

        // Create the module
        let shader_module = device.create_shader_module(
            &self
                .shader_source
                .take()
                .context("No shader source supplied!")?,
        );

        // Create the actual pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(self.pipeline_name),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: self.vertex_shader_entry,
                buffers: &[crate::Vertex::layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: self.primitive_topology,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
                // Setting this to true requires Features::DEPTH_CLAMPING
                clamp_depth: false,
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: self.fragment_shader_entry,
                targets: &[wgpu::ColorTargetState {
                    format: self.texture_format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
        });

        Ok(pipeline)
    }
}
