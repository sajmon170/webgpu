use glam::{Mat3A, Vec2};

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

    pub fn set_render_pass(&mut self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        self.material.set_transform(self.transform);
        self.material.set_render_pass(render_pass, queue);
        self.mesh.set_render_pass(render_pass);
    }

    pub fn translate(&mut self, translation: Vec2) {
        self.transform *= Mat3A::from_translation(translation);
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.transform *= Mat3A::from_rotation_z(rotation);
    }

    pub fn scale(&mut self, scale: Vec2) {
        self.transform *= Mat3A::from_scale(scale);
    }
}
