use std::sync::Arc;

use vulkano::{
    device::Device,
    format::Format,
    image::{ImageUsage, view::ImageView},
    swapchain::{ColorSpace, PresentMode, Surface, Swapchain, SwapchainCreateInfo},
};
use winit::window::Window;

pub struct SwapchainBundle {
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<ImageView>>,
    image_index: usize,
    recreate_swapchain: bool,
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

        SwapchainBundle {
            swapchain,
            images,
            image_index: 0,
            recreate_swapchain: false,
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
        &self.images[self.image_index]
    }
}
