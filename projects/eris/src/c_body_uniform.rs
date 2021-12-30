use crevice::std140::AsStd140;
use vesta::{
    cgmath::{Matrix3, Matrix4},
    components::Transform,
};

use crate::c_body::CelestialBodySettings;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsStd140)]
pub struct CelestialBodyDetails {
    pub model: Matrix4<f32>,  // 4x4 matrix
    pub normal: Matrix3<f32>, // 3x3 matrix
    pub temp_k: f32,
    pub atmosphere_density: f32,
}

unsafe impl vesta::bytemuck::Zeroable for CelestialBodyDetails {}
unsafe impl vesta::bytemuck::Pod for CelestialBodyDetails {}

impl CelestialBodyDetails {
    pub fn new(transform: Transform<f32>, settings: CelestialBodySettings) -> Self {
        Self {
            model: transform.calculate_model_matrix(),
            normal: transform.calculate_normal_matrix(),
            temp_k: settings.temp_k,
            atmosphere_density: settings.atmosphere_density,
        }
    }
}

pub struct CelestialBodyPipeline {
    pub render_pipeline: vesta::wgpu::RenderPipeline,
    pub outline_render_pipeline: vesta::wgpu::RenderPipeline,
}

impl CelestialBodyPipeline {
    pub fn new(engine: &vesta::Engine) -> Self {
        let noise_shader_src = include_str!("shaders/noise/noise.wgsl");
        let c_body_shader_src = include_str!("shaders/c_body.wgsl");

        let c_body_shader_final_src = [noise_shader_src, c_body_shader_src].join("\n");

        // Layout for this pipeline
        let render_pipeline_layout =
            engine
                .renderer
                .device
                .create_pipeline_layout(&vesta::wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &vesta::Texture::create_bind_group_layout(&engine.renderer.device),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX,
                            &engine.renderer.device,
                        ),
                        &vesta::UniformBufferUtils::create_bind_group_layout(
                            vesta::wgpu::ShaderStages::VERTEX | vesta::wgpu::ShaderStages::FRAGMENT,
                            &engine.renderer.device,
                        ),
                    ],
                    push_constant_ranges: &[],
                });

        // Main rendering pipeline for celestial bodies
        let render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "Celestial Body Shader",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            c_body_shader_final_src.clone().into(),
        ))
        .with_layout(&render_pipeline_layout)
        .build(&engine.renderer.device)
        .unwrap();

        // Identical to the render pipeline, but with a typology of LineList
        let outline_render_pipeline = vesta::RenderPipelineBuilder::new(
            engine.renderer.surface_config.format,
            "Celestial Body Shader (Outline)",
        )
        .with_shader_source(vesta::wgpu::ShaderSource::Wgsl(
            c_body_shader_final_src.clone().into(),
        ))
        .with_layout(&render_pipeline_layout)
        .with_topology(vesta::wgpu::PrimitiveTopology::LineList)
        .build(&engine.renderer.device)
        .unwrap();

        Self {
            render_pipeline,
            outline_render_pipeline,
        }
    }
}
