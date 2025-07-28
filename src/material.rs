use std::{default::Default, mem::size_of, num::NonZero};
use crate::{data::Vertex, gpu::Gpu};
use bytemuck::NoUninit;
use glam::{Mat3A, Vec2};

pub trait Material {
    fn set_render_pass(&self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue);
    // TODO: refactor transforms out of materials into a separate bind group
    // owned by Object
    fn set_transform(&mut self, transform: Mat3A);
}

#[repr(C, packed)]
#[derive(Copy, Clone, NoUninit)]
struct UniformData {
    pub xform: Mat3A,
    pub time: f32,
    _align: [u8; 16 - size_of::<f32>()],
}

pub struct MaterialBasic {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    start_time: std::time::Instant,
    // TODO - refactor this
    xform: Mat3A
}

impl MaterialBasic {
    fn setup_bind_group(device: &wgpu::Device, uniform_buffer: &wgpu::Buffer)
                        -> (wgpu::BindGroup, wgpu::PipelineLayout) {
        let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: "Bind group layout".into(),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }]
        };

        let bind_group_layout = device.create_bind_group_layout(&bind_group_layout_descriptor);

        let pipeline_descriptor = wgpu::PipelineLayoutDescriptor {
            label: "Uniform buffer layout".into(),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[]
        };

        let pipeline_layout = device.create_pipeline_layout(&pipeline_descriptor);

        let bind_group_descriptor = wgpu::BindGroupDescriptor {
            label: "Bind group".into(),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: NonZero::new(size_of::<UniformData>() as u64)
                })
            }]
        };

        let bind_group = device.create_bind_group(&bind_group_descriptor);

        (bind_group, pipeline_layout)
    }
    
    fn make_pipeline(device: &wgpu::Device,
                    config: &wgpu::SurfaceConfiguration,
                    pipeline_layout: &wgpu::PipelineLayout) -> wgpu::RenderPipeline {
        let shader_module = device.create_shader_module(
            wgpu::include_wgsl!("shader.wgsl")
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
                cull_mode: None,
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
            depth_stencil: None,
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
    
    pub fn new(gpu: &Gpu) -> Self {
        let uniform_buffer = Self::make_uniform_buffer(&gpu.device);
        let (bind_group, pipeline_layout) = Self::setup_bind_group(&gpu.device, &uniform_buffer);
        let pipeline = Self::make_pipeline(&gpu.device, &gpu.config, &pipeline_layout);
        let start_time = std::time::Instant::now();

        Self {
            bind_group,
            uniform_buffer,
            pipeline,
            start_time,
            xform: Mat3A::IDENTITY
        }
    }
}

impl Material for MaterialBasic {
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

    fn set_transform(&mut self, transform: Mat3A) {
        self.xform = transform;
    }
}
