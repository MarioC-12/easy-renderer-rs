mod commands;
mod context;
mod descriptors;
mod pipeline;
mod swapchain;

use std::{sync::Arc, time::Duration};

use glam::{Mat4, Vec3};
use vulkano::{command_buffer::PrimaryAutoCommandBuffer, sync::GpuFuture};
use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::{
    renderer::{
        commands::record_command_buffer, context::VulkanContext, descriptors::DescriptorBundle,
        pipeline::PipelineBundle, swapchain::SwapchainBundle,
    },
    resources::shaders::vs,
    scene::mesh::Mesh,
};

pub struct Renderer {
    window: Arc<Window>,
    context: VulkanContext,
    swapchain: SwapchainBundle,
    pipeline: PipelineBundle,
    descriptors: DescriptorBundle,
}

impl Renderer {
    pub fn new(window: Arc<Window>, event_loop: &ActiveEventLoop) -> Self {
        let context = VulkanContext::new(event_loop);
        let swapchain = SwapchainBundle::new(context.device(), &window);
        let pipeline = PipelineBundle::new(context.device(), &swapchain);
        let descriptors = DescriptorBundle::new(
            context.memory_allocator(),
            context.descriptor_set_allocator(),
            pipeline.pipeline(),
        );

        Renderer {
            window,
            context,
            swapchain,
            pipeline,
            descriptors,
        }
    }

    pub fn draw_frame(&mut self, mesh: &Mesh) {
        let future = self
            .swapchain
            .acquire(Some(Duration::from_secs(1)))
            .unwrap();

        self.swapchain.wait_for_current_frame_fence();

        let frame_index = self.swapchain.current_frame();
        let model = Mat4::IDENTITY.to_cols_array_2d();
        let view = Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
        .to_cols_array_2d();
        let proj =
            Mat4::perspective_rh(45.0_f32.to_radians(), 16.0 / 9.0, 0.1, 100.0).to_cols_array_2d();
        let mvp = vs::MVP { model, view, proj };

        self.descriptors.update_mvp(frame_index, mvp);

        let cmd_buffer = record_command_buffer(
            self.context.command_allocator(),
            self.context.graphics_queue().queue_family_index(),
            self.pipeline.pipeline(),
            self.swapchain.image_view(),
            self.descriptors.mvp_set(frame_index).clone(),
            mesh,
        );

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

    pub fn handle_resize(&mut self) {
        self.swapchain.request_recreate();
    }

    #[inline]
    pub fn context(&self) -> &VulkanContext {
        &self.context
    }
}
