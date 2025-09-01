use std::mem::size_of;
use crate::{Gpu, data::Vertex};

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl Mesh {
    const VERTICES: &[Vertex] = &[
        Vertex { pos: [-0.5, -0.5, -0.3 ], uv: [1.0, 1.0, 1.0] },
        Vertex { pos: [0.5, -0.5, -0.3 ], uv: [1.0, 1.0, 1.0] },
        Vertex { pos: [0.5, 0.5, -0.3 ], uv: [1.0, 1.0, 1.0] },
        Vertex { pos: [-0.5, 0.5, -0.3 ], uv: [1.0, 1.0, 1.0] },
        Vertex { pos: [0.0, 0.0, 0.5 ], uv: [1.0, 1.0, 1.0] },
    ];

    const INDICES: &[u16] = &[
        // Base
        0,  1,  2,
        0,  2,  3,
        // Sides/
        4,  1,  0,
        4,  2,  1,
        4,  3,  2,
        4,  0,  3,
    ];
    
    fn make_vertex_buffer(device: &wgpu::Device, vtx: &[Vertex]) -> wgpu::Buffer {
        let descriptor = wgpu::BufferDescriptor {
            label: "Vertex buffer".into(),
            size: (vtx.len() * size_of::<Vertex>()) as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX
        };

        device.create_buffer(&descriptor)
    }

    fn make_index_buffer(device: &wgpu::Device, idx: &[u16]) -> wgpu::Buffer {
        let descriptor = wgpu::BufferDescriptor {
            label: "Index buffer".into(),
            size: (idx.len() * size_of::<u16>()) as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::INDEX
        };

        device.create_buffer(&descriptor)
    }

    pub fn new(gpu: &Gpu, vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        let vertex_buffer = Self::make_vertex_buffer(&gpu.device, &vertices);
        gpu.queue.write_buffer(&vertex_buffer, 0, &bytemuck::cast_slice(&vertices));
        let index_buffer = Self::make_index_buffer(&gpu.device, &indices);
        gpu.queue.write_buffer(&index_buffer, 0, &bytemuck::cast_slice(&indices));
        
        Self {
            vertex_buffer,
            index_buffer,
            vertices,
            indices,
        }
    }

    pub fn new_debug(gpu: &Gpu) -> Self {
        Self::new(gpu, Self::VERTICES.into(), Self::INDICES.into())
    }

    pub fn set_render_pass(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
    }
}
