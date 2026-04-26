use std::sync::Arc;

use vulkano::{
    DeviceSize,
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{AutoCommandBufferBuilder, CopyBufferInfo, PrimaryAutoCommandBuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::graphics::vertex_input::Vertex,
};

#[derive(BufferContents, Vertex, Clone)]
#[repr(C)]
pub struct VertexT {
    #[format(R32G32_SFLOAT)]
    pub in_position: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    pub in_color: [f32; 3],
}

pub fn upload_buffer<T>(
    allocator: &Arc<StandardMemoryAllocator>,
    cbb: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    data: &[T],
    device_usage: BufferUsage,
) -> Result<Subbuffer<[T]>, Box<dyn std::error::Error>>
where
    T: BufferContents + Clone,
{
    let staging = Buffer::from_iter(
        allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data.iter().cloned(),
    )
    .unwrap();

    let device_buffer = Buffer::new_slice::<T>(
        allocator.clone(),
        BufferCreateInfo {
            usage: device_usage | BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
        data.len() as DeviceSize,
    )
    .unwrap();

    cbb.copy_buffer(CopyBufferInfo::buffers(staging, device_buffer.clone()))?;

    Ok(device_buffer)
}
