use anyhow::*;

pub struct RenderPipelineBuilder<'a> {
    layout: Option<&'a wgpu::PipelineLayout>,
    shader_source: Option<wgpu::ShaderModuleDescriptor<'a>>,
    vertex_shader_entry: &'a str,
    fragment_shader_entry: &'a str,
    texture_format: wgpu::TextureFormat,
    pipeline_name: &'a str,
    primitive_topology: wgpu::PrimitiveTopology,
    cull_mode: Option<wgpu::Face>,
    front_face: wgpu::FrontFace,
    vertex_buffer_layout: Option<&'a [wgpu::VertexBufferLayout<'a>]>,
    depth_write_enabled: bool,
    depth_compare: wgpu::CompareFunction,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(
        texture_format: wgpu::TextureFormat,
        pipeline_name: &'a str,
    ) -> RenderPipelineBuilder {
        Self {
            layout: None,
            shader_source: None,
            vertex_shader_entry: "main",
            fragment_shader_entry: "main",
            texture_format,
            pipeline_name,
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            front_face: wgpu::FrontFace::Ccw,
            vertex_buffer_layout: None,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
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

    #[allow(dead_code)]
    pub fn with_cull_mode(&mut self, face: Option<wgpu::Face>) -> &mut Self {
        self.cull_mode = face;
        self
    }

    #[allow(dead_code)]
    pub fn with_front_face(&mut self, front_face: wgpu::FrontFace) -> &mut Self {
        self.front_face = front_face;
        self
    }

    #[allow(dead_code)]
    pub fn with_vertex_buffer_layout(
        &mut self,
        layout: &'a [wgpu::VertexBufferLayout<'a>],
    ) -> &mut Self {
        self.vertex_buffer_layout = Some(layout);
        self
    }

    #[allow(dead_code)]
    pub fn with_depth_write_enabled(&mut self, depth_write_enabled: bool) -> &mut Self {
        self.depth_write_enabled = depth_write_enabled;
        self
    }

    #[allow(dead_code)]
    pub fn with_depth_compare(&mut self, depth_compare: wgpu::CompareFunction) -> &mut Self {
        self.depth_compare = depth_compare;
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

        // I really don't like this, but I don't know how to do this correctly in rust,
        // may need to rewrite this entire class...
        let mut buffers: &[wgpu::VertexBufferLayout] = &[crate::Vertex::layout()];
        if self.vertex_buffer_layout.is_some() {
            // Use the provided
            buffers = self.vertex_buffer_layout.unwrap()
        }

        // Create the actual pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(self.pipeline_name),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: self.vertex_shader_entry,
                buffers,
            },
            primitive: wgpu::PrimitiveState {
                topology: self.primitive_topology,
                strip_index_format: None,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Setting this to true requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: self.depth_write_enabled,
                depth_compare: self.depth_compare,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
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
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
        });

        Ok(pipeline)
    }
}
