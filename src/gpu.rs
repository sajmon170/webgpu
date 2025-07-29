use winit::{window::Window, dpi::PhysicalSize};
use anyhow::Result;
use std::sync::Arc;

pub struct Gpu {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl Gpu {
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
        //limits.max_buffer_size = (Gpu::VERTICES.len() * size_of::<Vertex>()) as u64;
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

    pub async fn new(window: Window, size: PhysicalSize<u32>) -> Result<Self> {
        let window = Arc::new(window);
        
        let instance = Self::get_instance();
        let surface = instance.create_surface(window.clone())?;
        let adapter = Self::get_adapter(&instance, &surface).await?;
        let limits = Self::get_limits();
        let (device, queue) = Self::get_device(&adapter, limits).await?;

        let config = Self::get_config(&adapter, &surface, size);
        surface.configure(&device, &config);

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
    }

    pub fn render(&self, mut set_render_pass: impl FnMut(&mut wgpu::RenderPass)) -> Result<()> {
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
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.1,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            set_render_pass(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.window.request_redraw();
        Ok(())
    }
}
