use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct VertexT {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}

const VERTICES: [VertexT; 3] = [
    VertexT {
        position: [0.0, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    VertexT {
        position: [0.5, 0.5],
        color: [1.0, 0.0, 0.0],
    },
    VertexT {
        position: [-0.5, 0.5],
        color: [1.0, 0.0, 0.0],
    },
];
