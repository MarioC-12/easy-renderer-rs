mod commands;
mod context;
mod pipeline;
mod swapchain;

use std::{sync::Arc, time::Duration};

use vulkano::sync::GpuFuture;
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

    pub fn draw_frame(&mut self) {
        let cmd_buffer = record_command_buffer(
            self.context.command_allocator(),
            self.context.graphics_queue().queue_family_index(),
            self.pipeline.pipeline(),
            self.swapchain.image_view(),
        );

        let future = self
            .swapchain
            .acquire(Some(Duration::from_secs(1)))
            .unwrap();

        let future = future
            .then_execute(self.context.graphics_queue().clone(), cmd_buffer)
            .unwrap()
            .boxed();

        self.swapchain.present(
            self.context.device(),
            future,
            self.context.graphics_queue().clone(),
            false,
        );
    }
}
