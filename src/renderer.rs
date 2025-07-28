use crate::{gpu::Gpu, material::MaterialBasic, mesh::Mesh, object::Object};
use winit::dpi::PhysicalSize;
use anyhow::Result;
use glam::Vec2;

pub struct Renderer {
    gpu: Gpu,
    objects: Vec<Object>
}

impl Renderer {
    pub fn render(&mut self) -> Result<()> {
        self.gpu.render(|render_pass| {
            for object in &mut self.objects {
                object.set_render_pass(render_pass, &self.gpu.queue);
            }
        })
    }

    pub fn new(gpu: Gpu) -> Self {
        let mesh = Mesh::new_debug(&gpu);
        let material = MaterialBasic::new(&gpu);
        let mut obj0 = Object::new(mesh, material);
        obj0.translate(Vec2::new(-0.3, -1.5));
        obj0.rotate(0.5);
        
        let mesh = Mesh::new_debug(&gpu);
        let material = MaterialBasic::new(&gpu);
        let mut obj1 = Object::new(mesh, material);
        obj1.scale(Vec2::new(0.2, 0.2));
        obj1.translate(Vec2::new(5.0, 0.0));
        obj1.rotate(0.9);

        let mesh = Mesh::new_debug(&gpu);
        let material = MaterialBasic::new(&gpu);
        let mut obj2 = Object::new(mesh, material);
        obj2.scale(Vec2::new(0.1, 0.1));
        obj2.translate(Vec2::new(-7.0, 10.0));
        obj2.rotate(4.0);

        let mesh = Mesh::new_debug(&gpu);
        let material = MaterialBasic::new(&gpu);
        let mut obj3 = Object::new(mesh, material);
        obj3.scale(Vec2::new(0.75, 0.75));
        obj3.translate(Vec2::new(0.0, 2.5));
        obj3.rotate(1.0);

        let mesh = Mesh::new_debug(&gpu);
        let material = MaterialBasic::new(&gpu);
        let mut obj4 = Object::new(mesh, material);
        obj4.scale(Vec2::new(0.6, 0.6));
        obj4.translate(Vec2::new(5.0, 3.5));
        obj4.rotate(0.4);

        let mesh = Mesh::new_debug(&gpu);
        let material = MaterialBasic::new(&gpu);
        let mut obj5 = Object::new(mesh, material);
        obj5.scale(Vec2::new(0.2, 0.2));
        obj5.translate(Vec2::new(7.0, 10.0));
        obj4.rotate(2.0);
        
        Self { gpu, objects: vec![obj0, obj1, obj2, obj3, obj4, obj5] }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
    }
}
