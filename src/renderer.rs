use crate::{gpu::Gpu, material::SimpleMaterial, mesh::Mesh, object::Object};
use winit::dpi::PhysicalSize;
use anyhow::Result;
use glam::{Vec2, Vec3};

pub struct Renderer {
    gpu: Gpu,
    objects: Vec<Object>
}

impl Renderer {
    pub fn render(&mut self) -> Result<()> {
        self.gpu.render(|render_pass| {
            for object in &mut self.objects {
                object.rotate_z(0.0001);
                object.set_render_pass(render_pass, &self.gpu.queue);
            }
        })
    }

    pub fn new(gpu: Gpu) -> Self {
        let mesh = Mesh::new_debug(&gpu);
        let material = SimpleMaterial::new(&gpu);
        let mut obj0 = Object::new(mesh, material);
        obj0.rotate_x(-1.1);
        obj0.scale(Vec3::new(0.5, 0.5, 1.0));
        obj0.translate(Vec3::new(0.0, 0.0, -0.2));

        Self { gpu, objects: vec![obj0] }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
    }
}
