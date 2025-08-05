use std::{default::Default, mem::size_of, num::NonZero};
use crate::{data::Vertex, gpu::Gpu};
use bytemuck::NoUninit;
use glam::{Mat4, Vec2};
use wgpu::{Extent3d, TexelCopyBufferLayout};

pub trait Material {
    fn set_render_pass(&self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue);
    // TODO: refactor transforms out of materials into a separate bind group
    // owned by Object
    fn set_transform(&mut self, transform: Mat4);
}

#[repr(C, packed)]
#[derive(Copy, Clone, NoUninit)]
struct UniformData {
    pub xform: Mat4,
    pub time: f32,
    _align: [u8; 16 - size_of::<f32>()],
}

pub struct SimpleMaterial {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    start_time: std::time::Instant,
    // TODO - refactor this
    xform: Mat4,
    texture: wgpu::Texture,
}

impl SimpleMaterial {
    fn setup_bind_group(device: &wgpu::Device, uniform_buffer: &wgpu::Buffer, texture: &wgpu::Texture)
                        -> (wgpu::BindGroup, wgpu::PipelineLayout) {
        let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: "Star material bind group layout".into(),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: true
                        },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                }
            ]
        };

        let bind_group_layout = device.create_bind_group_layout(&bind_group_layout_descriptor);

        let pipeline_descriptor = wgpu::PipelineLayoutDescriptor {
            label: "Uniform buffer layout".into(),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[]
        };

        let pipeline_layout = device.create_pipeline_layout(&pipeline_descriptor);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler_descriptor = wgpu::SamplerDescriptor {
            label: "Star texture sampler".into(),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };
        let sampler = device.create_sampler(&sampler_descriptor);

        let bind_group_descriptor = wgpu::BindGroupDescriptor {
            label: "Bind group".into(),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: NonZero::new(size_of::<UniformData>() as u64)
                    })
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view)
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler)
                }
            ]
        };

        let bind_group = device.create_bind_group(&bind_group_descriptor);

        (bind_group, pipeline_layout)
    }
 
    fn make_pipeline(device: &wgpu::Device,
                     config: &wgpu::SurfaceConfiguration,
                     pipeline_layout: &wgpu::PipelineLayout) -> wgpu::RenderPipeline {
        let shader_module = device.create_shader_module(
            wgpu::include_wgsl!("shaders/simple.wgsl")
        );

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle render"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3]
                    }
                ]
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })],
            }),
            depth_stencil: Some(wgpu::DepthStencilState {
                // TODO - grab this info from outside
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0u64,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None
        })
    }

    fn make_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        let descriptor = wgpu::BufferDescriptor {
            label: "Uniform buffer".into(),
            // TODO: round the size to a multiple of 4 f32 values
            size: size_of::<UniformData>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM
        };

        device.create_buffer(&descriptor)
    }

    fn make_texture(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture {
        let texture_bytes = include_bytes!("res/star.png");
        let texture_rgba = image::load_from_memory(texture_bytes).unwrap()
            .to_rgba8();
        let (tex_width, tex_height) = texture_rgba.dimensions();
        let extent = Extent3d {
            width: tex_width,
            height: tex_height,
            depth_or_array_layers: 1
        };
        
        let descriptor = wgpu::TextureDescriptor {
            label: "Star texture".into(),
            dimension: wgpu::TextureDimension::D2,
            size: extent,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            sample_count: 1,
            mip_level_count: 1,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        };

        let texture = device.create_texture(&descriptor);

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            },
            &texture_rgba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(tex_width * 4),
                rows_per_image: Some(tex_height)
            },
            extent
        );

        texture
    }
    
    pub fn new(gpu: &Gpu) -> Self {
        let uniform_buffer = Self::make_uniform_buffer(&gpu.device);
        let texture = Self::make_texture(&gpu.device, &gpu.queue);
        let (bind_group, pipeline_layout) = Self::setup_bind_group(&gpu.device, &uniform_buffer, &texture);
        let pipeline = Self::make_pipeline(&gpu.device, &gpu.config, &pipeline_layout);
        let start_time = std::time::Instant::now();

        Self {
            bind_group,
            uniform_buffer,
            pipeline,
            start_time,
            xform: Mat4::IDENTITY,
            texture,
        }
    }
}

impl Material for SimpleMaterial {
    fn set_render_pass(&self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        let time = std::time::Instant::now()
            .duration_since(self.start_time)
            .as_secs_f32();

        let uniform_data = UniformData {
            xform: self.xform,
            time,
            _align: Default::default(),
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform_data));
    }

    fn set_transform(&mut self, transform: Mat4) {
        self.xform = transform;
    }
}
