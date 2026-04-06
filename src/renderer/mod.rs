mod commands;
mod context;
mod pipeline;
mod swapchain;

use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::renderer::{
    commands::record_command_buffer, context::VulkanContext, pipeline::PipelineBundle,
    swapchain::SwapchainBundle,
};

pub struct Renderer {
    window: Arc<Window>,
    context: VulkanContext,
    swapchain: SwapchainBundle,
    pipeline: PipelineBundle,
}

impl Renderer {
    pub fn new(window: Arc<Window>, event_loop: &ActiveEventLoop) -> Self {
        let context = VulkanContext::new(event_loop);
        let swapchain = SwapchainBundle::new(context.device(), &window);
        let pipeline = PipelineBundle::new(context.device(), &swapchain);
        let command_b = record_command_buffer(
            context.command_allocator(),
            context.graphics_queue().queue_family_index(),
            pipeline.pipeline(),
            swapchain.image_view(),
        );

        Renderer {
            window,
            context,
            swapchain,
            pipeline,
        }
    }
}
