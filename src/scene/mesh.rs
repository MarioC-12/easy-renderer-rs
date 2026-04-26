use std::sync::Arc;

use vulkano::{
    buffer::{BufferUsage, Subbuffer},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
        allocator::StandardCommandBufferAllocator,
    },
    device::Queue,
    memory::allocator::StandardMemoryAllocator,
    sync::GpuFuture,
};

use crate::resources::buffers::{VertexT, upload_buffer};

pub struct Mesh {
    vertex_buffer: Subbuffer<[VertexT]>,
    index_buffer: Subbuffer<[u32]>,
}

impl Mesh {
    pub fn new(
        allocator: &Arc<StandardMemoryAllocator>,
        command_allocator: &Arc<StandardCommandBufferAllocator>,
        transfer_queue: &Arc<Queue>,
        vertices: &[VertexT],
        indexes: &[u32],
    ) -> Self {
        let mut cbb = AutoCommandBufferBuilder::primary(
            command_allocator.clone(),
            transfer_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let vertex_buffer =
            upload_buffer(allocator, &mut cbb, vertices, BufferUsage::VERTEX_BUFFER).unwrap();

        let index_buffer =
            upload_buffer(allocator, &mut cbb, indexes, BufferUsage::INDEX_BUFFER).unwrap();

        let cb = cbb.build().unwrap();

        cb.execute(transfer_queue.clone())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        Self {
            vertex_buffer,
            index_buffer,
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
