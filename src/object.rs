use std::path::Path;

use glam::{Mat4, Vec3, Vec4};
use tobj::LoadError;

use crate::{
    data::Vertex, gpu::Gpu, material::{Material, SimpleMaterial}, mesh::Mesh
};

// Refactor: Create a Camera entity that renders objects and provides
// view and projection transforms. Create a Scene entity that stores
// camera and objects

pub struct Object {
    meshes: Vec<Mesh>,
    material: Box<dyn Material>,
    // Refactor this
    projection_xform: Mat4,
    view_xform: Mat4,
    model_xform: Mat4,
}

impl Object {
    fn make_projection_matrix() -> Mat4 {
        let fov = 45.0 * std::f32::consts::PI / 180.0;
        let ratio = 640.0/480.0;
        let near = 0.01;
        let far = 100.0;
        
        Mat4::perspective_lh(fov, ratio, near, far)
    }

    fn make_view_matrix() -> Mat4 {
        Mat4::from_translation(Vec3::new(0.0, 0.0, 3.0))
    }

    pub fn load_obj(gpu: &Gpu, path: &Path) -> Result<Self, LoadError> {
        let (models, _) = tobj::load_obj(&path, &tobj::GPU_LOAD_OPTIONS)?;
        let mut meshes = Vec::<Mesh>::new();
 
        for model in models {
            let vertices: Vec<_> = model.mesh.positions.chunks_exact(3)
                .map(|pos| Vertex {
                    pos: [pos[0], -pos[2], pos[1]],
                    uv: [1.0, 1.0, 1.0]
                })
                .collect();

            meshes.push(Mesh::new(gpu, vertices, model.mesh.indices));
        }

        let material = Box::new(SimpleMaterial::new(&gpu));

        Ok(Self {
            meshes,
            material,
            projection_xform: Self::make_projection_matrix(),
            view_xform: Self::make_view_matrix(),
            model_xform: Mat4::IDENTITY
        })
    }

    pub fn set_render_pass(&mut self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        self.material.set_projection_xform(self.projection_xform);
        self.material.set_view_xform(self.view_xform);
        self.material.set_model_xform(self.model_xform);
        self.material.set_render_pass(render_pass, queue);

        for mesh in &self.meshes {
            mesh.set_render_pass(render_pass);
        }
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
