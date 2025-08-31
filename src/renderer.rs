use crate::{gpu::Gpu, material::SimpleMaterial, mesh::Mesh, object::Object};
use winit::dpi::PhysicalSize;
use anyhow::Result;
use glam::{Vec2, Vec3};

pub struct Renderer {
    begin: std::time::Instant,
    gpu: Gpu,
    objects: Vec<Object>
}

impl Renderer {
    pub fn render(&mut self) -> Result<()> {
        self.gpu.render(|render_pass| {
            for object in &mut self.objects {
                let time = std::time::Instant::now()
                    .duration_since(self.begin.clone())
                    .as_secs_f32();

                object.reset();
                object.translate(Vec3::new(0.0, 0.0, 2.0));
                object.rotate_x(-3.0 * 3.14159 / 4.0);
                object.rotate_z(time);
                object.translate(Vec3::new(1.0, 0.0, 0.0));
                
                object.set_render_pass(render_pass, &self.gpu.queue);
            }
        })
    }

    pub fn new(gpu: Gpu) -> Self {
        let mesh = Mesh::new_debug(&gpu);
        let material = SimpleMaterial::new(&gpu);
        let obj0 = Object::new(mesh, material);        
        let begin = std::time::Instant::now();

        Self { begin, gpu, objects: vec![obj0] }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
    }
}
