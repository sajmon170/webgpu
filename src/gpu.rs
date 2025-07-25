use winit::{window::Window, dpi::PhysicalSize};
use anyhow::Result;
use std::{sync::Arc, mem::size_of, num::NonZero};
use crate::data::Vertex;

pub struct Gpu {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    start_time: std::time::Instant
}

impl Gpu {
    const VERTICES: &[Vertex] = &[
        Vertex { pos: [ 0.0,    1.0,   0.0], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [-0.35,   0.45,  0.0], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [-1.0,    0.3,   0.0], color: [0.0, 0.0, 1.0] },
        Vertex { pos: [-0.55,  -0.15,  0.0], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [-0.6,   -0.9,   0.0], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [ 0.0,   -0.6,   0.0], color: [0.0, 0.0, 1.0] },
        Vertex { pos: [ 0.6,   -0.9,   0.0], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [ 0.55,  -0.15,  0.0], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [ 1.0,    0.3,   0.0], color: [0.0, 0.0, 1.0] },
        Vertex { pos: [ 0.35,   0.45,  0.0], color: [0.0, 1.0, 0.0] },
    ];

    const INDICES: &[u16] = &[
        0, 1, 9,
        1, 2, 3,
        3, 4, 5,
        7, 5, 6,
        9, 7, 8,
        1, 3, 5,
        9, 1, 5,
        9, 5, 7
    ]; 

    fn get_instance() -> wgpu::Instance {
        let descriptor = wgpu::InstanceDescriptor::default();
        wgpu::Instance::new(&descriptor)
    }

    fn get_config(
        adapter: &wgpu::Adapter,
        surface: &wgpu::Surface<'static>,
        size: PhysicalSize<u32>,
    ) -> wgpu::SurfaceConfiguration {
        let capabilities = surface.get_capabilities(&adapter);

        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn get_limits() -> wgpu::Limits {
        let mut limits = wgpu::Limits::defaults();
        limits.max_vertex_attributes = 2;
        limits.max_buffer_size = (Gpu::VERTICES.len() * size_of::<Vertex>()) as u64;
        limits.max_bind_groups = 1;

        limits
    }

    async fn get_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'static>,
    ) -> Result<wgpu::Adapter> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await?;

        Ok(adapter)
    }

    async fn get_device(adapter: &wgpu::Adapter, limits: wgpu::Limits)
                        -> Result<(wgpu::Device, wgpu::Queue)> {
        let mut descriptor = wgpu::DeviceDescriptor::default();
        descriptor.required_limits = limits;
        let (device, queue) = adapter.request_device(&descriptor).await?;

        device.set_device_lost_callback(|reason, message| {
            eprintln!("{:?}", reason);
            eprintln!("{message}");
        });

        queue.on_submitted_work_done(|| println!("Finished!"));

        Ok((device, queue))
    }

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
                    size: NonZero::new((4 * size_of::<f32>()) as u64)
                })
            }]
        };

        let bind_group = device.create_bind_group(&bind_group_descriptor);

        (bind_group, pipeline_layout)
    }

    fn get_pipeline(device: &wgpu::Device,
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

    fn make_vertex_buffer(device: &wgpu::Device, vtx: &[Vertex]) -> wgpu::Buffer {
        let descriptor = wgpu::BufferDescriptor {
            label: "Vertex buffer".into(),
            size: (vtx.len() * size_of::<Vertex>()) as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX
        };

        device.create_buffer(&descriptor)
    }

    fn make_index_buffer(device: &wgpu::Device, idx: &[u16]) -> wgpu::Buffer {
        let descriptor = wgpu::BufferDescriptor {
            label: "Index buffer".into(),
            size: (idx.len() * size_of::<u16>()) as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::INDEX
        };

        device.create_buffer(&descriptor)
    }

    fn make_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        let descriptor = wgpu::BufferDescriptor {
            label: "Uniform buffer".into(),
            // TODO: round the size to a multiple of 4 f32 values
            size: 4 * size_of::<f32>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM
        };

        device.create_buffer(&descriptor)
    }

    pub async fn new(window: Window, size: PhysicalSize<u32>) -> Result<Self> {
        let window = Arc::new(window);
        
        let instance = Self::get_instance();
        let surface = instance.create_surface(window.clone())?;
        let adapter = Self::get_adapter(&instance, &surface).await?;
        let limits = Self::get_limits();
        let (device, queue) = Self::get_device(&adapter, limits).await?;

        let vertex_buffer = Self::make_vertex_buffer(&device, Gpu::VERTICES);
        queue.write_buffer(&vertex_buffer, 0, &bytemuck::cast_slice(Gpu::VERTICES));

        let index_buffer = Self::make_index_buffer(&device, Gpu::INDICES);
        queue.write_buffer(&index_buffer, 0, &bytemuck::cast_slice(Gpu::INDICES));

        let uniform_buffer = Self::make_uniform_buffer(&device);

        let config = Self::get_config(&adapter, &surface, size);
        surface.configure(&device, &config);

        let (bind_group, pipeline_layout) = Self::setup_bind_group(&device, &uniform_buffer);

        let pipeline = Self::get_pipeline(&device, &config, &pipeline_layout);

        let start_time = std::time::Instant::now();

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group,
            start_time
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
    }

    pub fn render(&self) -> Result<()> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
 
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.7,
                            g: 0.3,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.bind_group, &[]);

            let time = std::time::Instant::now()
                .duration_since(self.start_time)
                .as_secs_f32();
            
            self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&time));

            render_pass.draw_indexed(0..Gpu::INDICES.len() as u32, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.window.request_redraw();
        Ok(())
    }
}
