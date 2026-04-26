use std::{
    sync::{Arc, atomic::fence},
    time::Duration,
};

use vulkano::{
    Validated, VulkanError,
    device::{Device, DeviceOwned, Queue},
    format::Format,
    image::{ImageUsage, view::ImageView},
    swapchain::{
        self, ColorSpace, PresentMode, Surface, Swapchain, SwapchainCreateInfo,
        SwapchainPresentInfo,
    },
    sync::{self, GpuFuture, future::FenceSignalFuture},
};
use winit::window::Window;

pub const FRAMES_IN_FLIGHT: usize = 2;

pub struct SwapchainBundle {
    window: Arc<Window>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<ImageView>>,
    present_mode: PresentMode,
    recreate_swapchain: bool,
    image_index: u32,
    current_frame: usize,
    previous_frame_ends: Vec<Option<FenceSignalFuture<Box<dyn GpuFuture>>>>,
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

        let previous_frame_ends: Vec<Option<FenceSignalFuture<Box<dyn GpuFuture>>>> =
            (0..FRAMES_IN_FLIGHT).map(|_| None).collect();

        SwapchainBundle {
            window: window.clone(),
            swapchain,
            images,
            present_mode,
            image_index: 0,
            recreate_swapchain: false,
            current_frame: 0,
            previous_frame_ends,
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
    fn recreate_swapchain(&mut self) {
        let image_extent: [u32; 2] = self.window.inner_size().into();

        if image_extent.contains(&0) {
            return;
        }

        let (new_swapchain, new_images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent,
                present_mode: self.present_mode,
                ..self.swapchain.create_info()
            })
            .expect("failed to recreate swapchain");

        self.swapchain = new_swapchain;

        let new_images = new_images
            .into_iter()
            .map(|image| ImageView::new_default(image).unwrap())
            .collect::<Vec<_>>();

        self.images = new_images;
    }

    #[inline]
    pub fn acquire(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<Box<dyn GpuFuture>, VulkanError> {
        if self.recreate_swapchain {
            self.recreate_swapchain();
            self.recreate_swapchain = false;
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

        let prev = self.previous_frame_ends[self.current_frame]
            .take()
            .map(|f| f.boxed() as Box<dyn GpuFuture>)
            .unwrap_or_else(|| sync::now(self.swapchain.device().clone()).boxed());

        Ok(prev.join(acquire_future).boxed())
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
            .boxed()
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(mut future) => {
                future.cleanup_finished();
                self.previous_frame_ends[self.current_frame] = Some(future);
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_ends[self.current_frame] = None;
            }
            Err(e) => {
                panic!("failed to flush future: {e}");
            }
        }

        self.current_frame = (self.current_frame + 1) % FRAMES_IN_FLIGHT;
    }

    #[inline]
    pub fn request_recreate(&mut self) {
        self.recreate_swapchain = true;
    }

    #[inline]
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    #[inline]
    pub fn wait_for_current_frame_fence(&self) {
        if let Some(ref fence) = self.previous_frame_ends[self.current_frame] {
            fence.wait(None).unwrap();
        }
    }
}
