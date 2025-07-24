use bytemuck::NoUninit;

#[repr(C)]
#[derive(Copy, Clone, Debug, NoUninit)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3]
}
