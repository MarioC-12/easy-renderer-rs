use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
};

use crate::resources::buffers::VertexT;

pub struct Mesh {
    vertex_buffer: Subbuffer<[VertexT]>,
}

impl Mesh {
    pub fn new(allocator: &Arc<StandardMemoryAllocator>, vertices: &[VertexT]) -> Self {
        Self {
            vertex_buffer: Buffer::from_iter(
                allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                vertices.iter().cloned(),
            )
            .unwrap(),
        }
    }

    pub fn vertex_buffer(&self) -> &Subbuffer<[VertexT]> {
        &self.vertex_buffer
    }

    pub fn num_vertices(&self) -> u32 {
        self.vertex_buffer.len() as u32
    }
}
