use std::sync::Arc;
use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
        RenderingAttachmentInfo, RenderingInfo, allocator::StandardCommandBufferAllocator,
    },
    format::ClearValue,
    image::view::ImageView,
    pipeline::GraphicsPipeline,
    render_pass::{AttachmentLoadOp, AttachmentStoreOp},
};

pub fn record_command_buffer(
    allocator: &Arc<StandardCommandBufferAllocator>,
    queue_family_index: u32,
    pipeline: &Arc<GraphicsPipeline>,
    swapchain_image_view: &Arc<ImageView>,
) -> Arc<PrimaryAutoCommandBuffer> {
    let mut builder = AutoCommandBufferBuilder::primary(
        allocator.clone(),
        queue_family_index,
        CommandBufferUsage::MultipleSubmit,
    )
    .unwrap();

    builder
        .begin_rendering(RenderingInfo {
            render_area_extent: swapchain_image_view.image().extent()[0..2]
                .try_into()
                .unwrap(),
            layer_count: 1,
            color_attachments: vec![Some(RenderingAttachmentInfo {
                load_op: AttachmentLoadOp::Clear,
                store_op: AttachmentStoreOp::Store,
                clear_value: Some(ClearValue::Float([0.0, 0.0, 0.0, 1.0])),
                ..RenderingAttachmentInfo::image_view(swapchain_image_view.clone())
            })],
            ..Default::default()
        })
        .unwrap()
        .bind_pipeline_graphics(pipeline.clone())
        .unwrap();

    unsafe { builder.draw(3, 1, 0, 0) }
        .unwrap()
        .end_rendering()
        .unwrap();

    builder.build().unwrap()
}
