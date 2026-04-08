use std::{sync::Arc, time::Duration};

use vulkano::{
    Validated, VulkanError,
    device::{Device, Queue},
    format::Format,
    image::{ImageUsage, view::ImageView},
    swapchain::{
        self, ColorSpace, PresentMode, Surface, Swapchain, SwapchainCreateInfo,
        SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
};
use winit::window::Window;

pub struct SwapchainBundle {
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<ImageView>>,
    recreate_swapchain: bool,
    image_index: u32,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

impl SwapchainBundle {
    pub fn new(device: &Arc<Device>, window: &Arc<Window>) -> Self {
        let surface = Surface::from_window(device.instance().clone(), window.clone()).unwrap();

        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .expect("failed to get surface capabilities");

        let image_format = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()
            .iter()
            .min_by_key(|(f, c)| match (f, c) {
                //This color space is actually the default so not really needed
                (Format::B8G8R8A8_SRGB, ColorSpace::ExtendedSrgbNonLinear) => 0,
                _ => 1,
            })
            .expect("no correct format found")
            .0;

        let composite_alpha = surface_capabilities
            .supported_composite_alpha
            .into_iter()
            .next()
            .unwrap();

        let present_mode = surface_capabilities
            .compatible_present_modes
            .into_iter()
            .min_by_key(|p| match p {
                PresentMode::Mailbox => 0,
                PresentMode::Fifo => 1,
                _ => 2,
            })
            .unwrap_or(PresentMode::Fifo);

        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count + 1,
                image_format,
                present_mode,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap();

        let images = images
            .into_iter()
            .map(|image| ImageView::new_default(image).unwrap())
            .collect::<Vec<_>>();

        let previous_frame_end = Some(sync::now(device.clone()).boxed());

        SwapchainBundle {
            swapchain,
            images,
            image_index: 0,
            recreate_swapchain: false,
            previous_frame_end,
        }
    }

    #[inline]
    pub fn image_format(&self) -> Format {
        self.swapchain.image_format()
    }

    #[inline]
    pub fn surface(&self) -> Arc<Surface> {
        self.swapchain.surface().clone()
    }

    #[inline]
    pub fn resize(&mut self) {
        self.recreate_swapchain = true;
    }

    #[inline]
    pub fn extent(&self) -> [f32; 2] {
        let [w, h] = self.swapchain.image_extent();
        [w as f32, h as f32]
    }

    #[inline]
    pub fn image_view(&self) -> &Arc<ImageView> {
        &self.images[self.image_index as usize]
    }

    #[inline]
    pub fn acquire(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<Box<dyn GpuFuture>, VulkanError> {
        if self.recreate_swapchain {
            //TODO: Recrate swapchain
        }

        let (image_index, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), timeout)
                .map_err(Validated::unwrap)
            {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return Err(VulkanError::OutOfDate);
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        self.image_index = image_index;
        let future = self.previous_frame_end.take().unwrap().join(acquire_future);

        Ok(future.boxed())
    }

    #[inline]
    pub fn present(
        &mut self,
        device: &Arc<Device>,
        after_future: Box<dyn GpuFuture>,
        queue: Arc<Queue>,
        wait_future: bool,
    ) {
        let future = after_future
            .then_swapchain_present(
                queue,
                SwapchainPresentInfo::swapchain_image_index(
                    self.swapchain.clone(),
                    self.image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(mut future) => {
                future.cleanup_finished();
                self.previous_frame_end = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(device.clone()).boxed());
            }
            Err(e) => {
                panic!("failed to flush future: {e}");
            }
        }
    }
}
