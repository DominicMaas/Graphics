use std::num::NonZeroU32;

use anyhow::*;
use image::GenericImageView;

use crate::renderer::Renderer;

pub struct TextureConfig {
    pub sampler_address_mode_u: wgpu::AddressMode,
    pub sampler_address_mode_v: wgpu::AddressMode,
    pub sampler_address_mode_w: wgpu::AddressMode,
    pub sampler_mag_filter: wgpu::FilterMode,
    pub sampler_min_filter: wgpu::FilterMode,
    pub sampler_mipmap_filter: wgpu::FilterMode,
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            sampler_address_mode_u: wgpu::AddressMode::ClampToEdge,
            sampler_address_mode_v: wgpu::AddressMode::ClampToEdge,
            sampler_address_mode_w: wgpu::AddressMode::ClampToEdge,
            sampler_mag_filter: wgpu::FilterMode::Linear,
            sampler_min_filter: wgpu::FilterMode::Nearest,
            sampler_mipmap_filter: wgpu::FilterMode::Nearest,
        }
    }
}

/// Represents a texture inside this application
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl Renderer {
    pub fn create_texture_from_bytes(
        &self,
        bytes: &[u8],
        label: Option<&str>,
        config: TextureConfig,
    ) -> Result<Texture> {
        Texture::from_bytes(&self.device, &self.queue, bytes, label, config)
    }

    pub fn create_texture_from_image(
        &self,
        image: &image::DynamicImage,
        label: Option<&str>,
        config: TextureConfig,
    ) -> Result<Texture> {
        Texture::from_image(&self.device, &self.queue, image, label, config)
    }

    /// Create a depth texture. This is a special type of texture that can be used for the
    /// depth buffer.
    pub fn create_depth_texture(&self, label: Option<&str>) -> Result<Texture> {
        Texture::create_depth(&self.device, &self.swap_chain_desc, label)
    }
}

impl Texture {
    // The DEPTH texture format used for this application
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// Create a texture from bytes
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: Option<&str>,
        config: TextureConfig,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, label, config)
    }

    /// Create a texture from an image
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
        config: TextureConfig,
    ) -> Result<Self> {
        let rgba = img.as_rgba8().unwrap();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: config.sampler_address_mode_u,
            address_mode_v: config.sampler_address_mode_v,
            address_mode_w: config.sampler_address_mode_w,
            mag_filter: config.sampler_mag_filter,
            min_filter: config.sampler_min_filter,
            mipmap_filter: config.sampler_mipmap_filter,
            ..Default::default()
        });

        // Create the appropriate bind group for the input data
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Self::create_bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        Ok(Self {
            texture,
            view,
            sampler,
            bind_group: Some(bind_group),
        })
    }

    /// Create a depth texture. This is a special type of texture that can be used for the
    /// depth buffer.
    pub fn create_depth(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        label: Option<&str>,
    ) -> Result<Self> {
        // Size of depth texture should match the swap chain descriptor
        let size = wgpu::Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth_or_array_layers: 1,
        };

        // Build for descriptor for depth texture
        let texture_desc = wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2, // 2D texture
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        };

        // Create the texture based on the descriptor
        let texture = device.create_texture(&texture_desc);

        // Create the view
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create the sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
            bind_group: None,
        })
    }

    pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: true,
                        comparison: false,
                    },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }
}
