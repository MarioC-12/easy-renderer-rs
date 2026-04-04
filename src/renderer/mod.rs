mod context;
mod pipeline;
mod swapchain;

use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::renderer::{
    context::VulkanContext, pipeline::PipelineBundle, swapchain::SwapchainBundle,
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

        Renderer {
            window,
            context,
            swapchain,
            pipeline,
        }
    }
}
