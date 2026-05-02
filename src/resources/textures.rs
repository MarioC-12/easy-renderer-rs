use std::sync::Arc;

use anyhow::Result;

use image::ImageReader;
use vulkano::{
    DeviceSize, VulkanError,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
        PrimaryCommandBufferAbstract, allocator::StandardCommandBufferAllocator,
    },
    device::Queue,
    format::Format,
    image::{Image, ImageCreateInfo, ImageType, ImageUsage, sampler::Sampler, view::ImageView},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    sync::GpuFuture,
};

fn upload_image(
    allocator: &Arc<StandardMemoryAllocator>,
    cbb: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    url: &str,
) -> Result<Arc<Image>> {
    let img = image::open(url)?.to_rgba8();
    let pixels = img.as_raw();
    let (width, height) = img.dimensions();

    let staging = Buffer::new_slice(
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
        pixels.len() as DeviceSize,
    )?;

    staging.write()?.copy_from_slice(pixels);

    let image = Image::new(
        allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_SRGB,
            extent: [width, height, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
    )?;

    cbb.copy_buffer_to_image(
        vulkano::command_buffer::CopyBufferToImageInfo::buffer_image(staging, image.clone()),
    )?;

    Ok(image)
}

pub struct Texture {
    view: Arc<ImageView>,
    sampler: Arc<Sampler>,
}

impl Texture {
    fn new(
        allocator: &Arc<StandardMemoryAllocator>,
        cbb: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        url: &str,
        sampler: &Arc<Sampler>,
    ) -> Result<Self> {
        let image = upload_image(allocator, cbb, url)?;

        let view = ImageView::new_default(image)?;

        Ok(Self {
            view,
            sampler: sampler.clone(),
        })
    }

    pub fn load_texture(
        allocator: &Arc<StandardMemoryAllocator>,
        command_allocator: &Arc<StandardCommandBufferAllocator>,
        transfer_queue: &Arc<Queue>,
        url: &str,
        sampler: &Arc<Sampler>,
    ) -> Result<Self> {
        let mut cbb = AutoCommandBufferBuilder::primary(
            command_allocator.clone(),
            transfer_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        let res = Self::new(allocator, &mut cbb, url, sampler)?;

        cbb.build()?
            .execute(transfer_queue.clone())?
            .then_signal_fence_and_flush()?
            .wait(None)?;

        Ok(res)
    }

    #[inline]
    pub fn view(&self) -> &Arc<ImageView> {
        &self.view
    }

    #[inline]
    pub fn sampler(&self) -> &Arc<Sampler> {
        &self.sampler
    }
}
