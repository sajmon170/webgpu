use bytemuck::NoUninit;

#[repr(C)]
#[derive(Copy, Clone, Debug, NoUninit)]
pub struct Vertex {
    pub pos: [f32; 3]
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {pos: [x, y, z]}
    }
}

impl From<[f32; 3]> for Vertex {
    fn from(src: [f32; 3]) -> Self {
        Self {pos: src}
    }
}
