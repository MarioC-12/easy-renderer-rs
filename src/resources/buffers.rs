use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex, Clone)]
#[repr(C)]
pub struct VertexT {
    #[format(R32G32_SFLOAT)]
    pub in_position: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    pub in_color: [f32; 3],
}
