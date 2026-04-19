use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
};

use crate::resources::buffers::VertexT;

pub struct Mesh {
    vertex_buffer: Subbuffer<[VertexT]>,
    index_buffer: Subbuffer<[u32]>,
}

impl Mesh {
    pub fn new(
        allocator: &Arc<StandardMemoryAllocator>,
        vertices: &[VertexT],
        indexes: &[u32],
    ) -> Self {
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
            index_buffer: Buffer::from_iter(
                allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::INDEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                indexes.iter().cloned(),
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

    pub fn index_buffer(&self) -> &Subbuffer<[u32]> {
        &self.index_buffer
    }

    pub fn num_indexes(&self) -> u32 {
        self.index_buffer.len() as u32
    }
}
