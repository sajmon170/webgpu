use crate::{gpu::Gpu, material::SimpleMaterial, mesh::Mesh, object::Object};
use winit::dpi::PhysicalSize;
use anyhow::Result;
use glam::{Vec2, Vec3};
use std::path::Path;

pub struct Renderer {
    begin: std::time::Instant,
    gpu: Gpu,
    objects: Vec<Object>
}

impl Renderer {
    pub fn render(&mut self) -> Result<()> {
        self.gpu.render(|render_pass| {
            /*
            for object in &mut self.objects {
                let time = std::time::Instant::now()
                    .duration_since(self.begin.clone())
                    .as_secs_f32();

                object.reset();
                //object.translate(Vec3::new(0.0, 0.0, 2.0));
                object.rotate_x(-2.0 * 3.14159 / 4.0);
                object.rotate_z(time);
                object.scale(Vec3::new(0.7, 0.7, 0.7));
                //object.translate(Vec3::new(1.0, 0.0, 0.0));
                
                object.set_render_pass(render_pass, &self.gpu.queue);
            }
            */

            let time = std::time::Instant::now()
                .duration_since(self.begin.clone())
                .as_secs_f32();

            let obj0 = &mut self.objects[0];

            obj0.reset();
            obj0.translate(Vec3::new(-0.7, 0.0, 0.0));
            obj0.rotate_x(-2.0 * 3.14159 / 4.0);
            obj0.rotate_z(time);
            obj0.scale(Vec3::new(0.6, 0.6, 0.6));

            obj0.set_render_pass(render_pass, &self.gpu.queue);

            let obj1 = &mut self.objects[1];

            obj1.reset();
            obj1.scale(Vec3::new(0.8, 0.8, 0.8));
            obj1.translate(Vec3::new(0.9, 0.0, 0.0));
            obj1.rotate_x(-2.5 * 3.14159 / 4.0);
            obj1.rotate_z(time);

            obj1.set_render_pass(render_pass, &self.gpu.queue);

        })
    }

    pub fn new(gpu: Gpu) -> Self {
        let obj1 = Object::load_obj(&gpu, &Path::new("src/res/models/sus/sus.obj")).unwrap();
        let mut obj2 = Object::load_obj(&gpu, &Path::new("src/res/models/obamium/obamium.obj")).unwrap();
        let begin = std::time::Instant::now();

        Self { begin, gpu, objects: vec![obj1, obj2] }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
    }
}
