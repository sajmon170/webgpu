use glam::{Mat4, Vec3};

use crate::{
    material::Material,
    mesh::Mesh
};

pub struct Object {
    mesh: Mesh,
    material: Box<dyn Material>,
    transform: Mat4
}

impl Object {
    pub fn new(mesh: Mesh, material: impl Material + 'static) -> Self {
        Self {
            mesh,
            material: Box::new(material),
            transform: Mat4::IDENTITY
        }
    }

    pub fn set_render_pass(&mut self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        self.material.set_transform(self.transform);
        self.material.set_render_pass(render_pass, queue);
        self.mesh.set_render_pass(render_pass);
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.transform *= Mat4::from_translation(translation);
    }

    pub fn rotate_x(&mut self, rotation: f32) {
        self.transform *= Mat4::from_rotation_x(rotation);
    }

    pub fn rotate_y(&mut self, rotation: f32) {
        self.transform *= Mat4::from_rotation_y(rotation);
    }

    pub fn rotate_z(&mut self, rotation: f32) {
        self.transform *= Mat4::from_rotation_z(rotation);
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.transform *= Mat4::from_scale(scale);
    }
}
