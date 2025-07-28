use glam::Mat3A;

use crate::{
    material::Material,
    mesh::Mesh
};

pub struct Object {
    mesh: Mesh,
    material: Box<dyn Material>,
    transform: Mat3A
}

impl Object {
    pub fn new(mesh: Mesh, material: impl Material + 'static) -> Self {
        Self {
            mesh,
            material: Box::new(material),
            transform: Mat3A::IDENTITY
        }
    }

    pub fn set_render_pass(&self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        self.material.set_render_pass(render_pass, queue);
        self.mesh.set_render_pass(render_pass);
    }
}
