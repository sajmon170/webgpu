use crate::{gpu::Gpu, material::MaterialBasic, mesh::Mesh, object::Object};
use winit::dpi::PhysicalSize;
use anyhow::Result;

pub struct Renderer {
    gpu: Gpu,
    objects: Vec<Object>
}

impl Renderer {
    pub fn render(&self) -> Result<()> {
        self.gpu.render(|render_pass| {
            for object in &self.objects {
                object.set_render_pass(render_pass, &self.gpu.queue);
            }
        })
    }

    pub fn new(gpu: Gpu) -> Self {
        let mesh = Mesh::new_debug(&gpu);
        let material = MaterialBasic::new(&gpu);
        let obj = Object::new(mesh, material);
        Self { gpu, objects: vec![obj] }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
    }
}
