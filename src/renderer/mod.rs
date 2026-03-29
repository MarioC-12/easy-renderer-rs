mod context;

use winit::event_loop::EventLoop;

use crate::renderer::context::VulkanContext;

pub struct Renderer {
    context: VulkanContext,
}

impl Renderer {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        Renderer {
            context: VulkanContext::new(event_loop),
        }
    }
}
