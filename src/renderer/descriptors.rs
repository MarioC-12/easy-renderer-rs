use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer, allocator::SubbufferAllocator},
    descriptor_set::{
        DescriptorSet, WriteDescriptorSet,
        allocator::{DescriptorSetAllocator, StandardDescriptorSetAllocator},
    },
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{GraphicsPipeline, Pipeline},
};

use crate::{
    renderer::swapchain::FRAMES_IN_FLIGHT,
    resources::{shaders::vs, textures::Texture},
};

pub struct DescriptorBundle {
    mvp_buffers: Vec<Subbuffer<vs::MVP>>,
    sets: Vec<Arc<DescriptorSet>>,
}

impl DescriptorBundle {
    pub fn new(
        allocator: &Arc<StandardMemoryAllocator>,
        descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
        pipeline: &Arc<GraphicsPipeline>,
        texture: &Texture,
    ) -> Self {
        let layout = pipeline.layout().set_layouts().get(0).unwrap().clone();

        let mvp_buffers: Vec<Subbuffer<vs::MVP>> = (0..FRAMES_IN_FLIGHT)
            .map(|_| {
                Buffer::new_sized(
                    allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::UNIFORM_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect();

        let sets = mvp_buffers
            .iter()
            .map(|buf| {
                DescriptorSet::new(
                    descriptor_set_allocator.clone(),
                    layout.clone(),
                    [
                        WriteDescriptorSet::buffer(0, buf.clone()),
                        WriteDescriptorSet::image_view_sampler(
                            1,
                            texture.view().clone(),
                            texture.sampler().clone(),
                        ),
                    ],
                    [],
                )
                .unwrap()
            })
            .collect();

        Self { mvp_buffers, sets }
    }

    #[inline]
    pub fn update_mvp(&mut self, frame_index: usize, mvp: vs::MVP) {
        *self.mvp_buffers[frame_index].write().unwrap() = mvp;
    }

    #[inline]
    pub fn mvp_set(&self, frame_index: usize) -> &Arc<DescriptorSet> {
        &self.sets[frame_index]
    }
}
