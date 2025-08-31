use glam::{Mat4, Vec3, Vec4};

use crate::{
    material::Material,
    mesh::Mesh
};

// Refactor: Create a Camera entity that renders objects and provides
// view and projection transforms. Create a Scene entity that stores
// camera and objects

pub struct Object {
    mesh: Mesh,
    material: Box<dyn Material>,
    // Refactor this
    projection_xform: Mat4,
    view_xform: Mat4,
    model_xform: Mat4,
}

impl Object {
    fn make_projection_matrix() -> Mat4 {
        let ratio = 640.0/480.0;
        let focal_length = 2.0;
        let near = 0.01;
        let far = 100.0;
        
        Mat4::from_cols(
            Vec4::new(focal_length, 0.0, 0.0, 0.0),
            Vec4::new(0.0, focal_length * ratio, 0.0, 0.0),
            Vec4::new(0.0, 0.0, far / (far - near), 1.0),
            Vec4::new(0.0, 0.0, -far * near / (far - near), 0.0)
        )
    }

    fn make_view_matrix() -> Mat4 {
        Mat4::from_translation(Vec3::new(0.0, 0.0, 3.0))
    }
    
    pub fn new(mesh: Mesh, material: impl Material + 'static) -> Self {
        Self {
            mesh,
            material: Box::new(material),
            projection_xform: Self::make_projection_matrix(),
            view_xform: Self::make_view_matrix(),
            model_xform: Mat4::IDENTITY
        }
    }

    pub fn set_render_pass(&mut self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        self.material.set_projection_xform(self.projection_xform);
        self.material.set_view_xform(self.view_xform);
        self.material.set_model_xform(self.model_xform);
        self.material.set_render_pass(render_pass, queue);
        self.mesh.set_render_pass(render_pass);
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.model_xform *= Mat4::from_translation(translation);
    }

    pub fn rotate_x(&mut self, rotation: f32) {
        self.model_xform *= Mat4::from_rotation_x(rotation);
    }

    pub fn rotate_y(&mut self, rotation: f32) {
        self.model_xform *= Mat4::from_rotation_y(rotation);
    }

    pub fn rotate_z(&mut self, rotation: f32) {
        self.model_xform *= Mat4::from_rotation_z(rotation);
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.model_xform *= Mat4::from_scale(scale);
    }

    pub fn reset(&mut self) {
        self.model_xform = Mat4::IDENTITY;
    }
}
