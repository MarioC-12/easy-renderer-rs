use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    device::{Device, physical::PhysicalDevice},
    instance::{
        Instance, InstanceCreateFlags, InstanceCreateInfo,
        debug::{
            DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger,
            DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo,
        },
    },
    swapchain::Surface,
};
use winit::event_loop::EventLoop;

pub struct VulkanContext {
    _messenger: Option<DebugUtilsMessenger>,
    pub instance: Arc<Instance>,
    // pub physical_device: Arc<PhysicalDevice>,
    // pub device: Arc<Device>,
}

impl VulkanContext {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let (instance, _messenger) = Self::create_instance(event_loop);
        VulkanContext {
            instance,
            _messenger,
        }
    }

    fn create_instance(event_loop: &EventLoop<()>) -> (Arc<Instance>, Option<DebugUtilsMessenger>) {
        let library = VulkanLibrary::new().expect("no VulkanLibrary/DLL");

        //Validation layers activation
        let enable_validation = cfg!(debug_assertions);
        let layers: Vec<_> = if enable_validation {
            library
                .layer_properties()
                .unwrap()
                .filter(|l| l.name() == "VK_LAYER_KHRONOS_validation")
                .collect()
        } else {
            Vec::new()
        };

        let mut required_extensions = Surface::required_extensions(event_loop)
            .expect("Cannot get the required extensions for Vulkan");
        required_extensions.ext_debug_utils = enable_validation;

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_layers: layers.iter().map(|l| l.name().to_owned()).collect(),
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create instance");

        //Validation messenger
        let _messenger = enable_validation.then(|| unsafe {
            DebugUtilsMessenger::new(
                instance.clone(),
                DebugUtilsMessengerCreateInfo {
                    message_severity: DebugUtilsMessageSeverity::ERROR
                        | DebugUtilsMessageSeverity::WARNING
                        | DebugUtilsMessageSeverity::VERBOSE,
                    message_type: DebugUtilsMessageType::GENERAL
                        | DebugUtilsMessageType::VALIDATION
                        | DebugUtilsMessageType::PERFORMANCE,
                    ..DebugUtilsMessengerCreateInfo::user_callback(
                        DebugUtilsMessengerCallback::new(|severity, _msg_type, data| {
                            println!("[{severity:?}] {}", data.message);
                        }),
                    )
                },
            )
            .unwrap()
        });

        (instance, _messenger)
    }
}
