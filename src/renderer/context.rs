use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    command_buffer::allocator::StandardCommandBufferAllocator,
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, Queue, QueueCreateInfo,
        QueueFlags,
        physical::{PhysicalDevice, PhysicalDeviceType},
    },
    instance::{
        Instance, InstanceCreateFlags, InstanceCreateInfo,
        debug::{
            DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger,
            DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo,
        },
    },
    memory::allocator::StandardMemoryAllocator,
    swapchain::Surface,
};
use winit::event_loop::ActiveEventLoop;

pub struct VulkanContext {
    _messenger: Option<DebugUtilsMessenger>,
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

struct PhysicalDeviceInfo {
    physical_device: Arc<PhysicalDevice>,
    device_extensions: DeviceExtensions,
}

impl VulkanContext {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let (instance, _messenger) = Self::create_instance(event_loop);
        let phys_info = Self::select_physical_device(&instance);
        let (device, queue) = Self::create_logical_device(&phys_info, event_loop);
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        VulkanContext {
            _messenger,
            instance,
            physical_device: phys_info.physical_device,
            device,
            graphics_queue: queue,
            memory_allocator,
            command_buffer_allocator,
        }
    }

    fn create_instance(
        event_loop: &ActiveEventLoop,
    ) -> (Arc<Instance>, Option<DebugUtilsMessenger>) {
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

    fn select_physical_device(instance: &Arc<Instance>) -> PhysicalDeviceInfo {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let physical_device = instance
            .enumerate_physical_devices()
            .expect("failed to get any physical device")
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .min_by_key(|p| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .expect("failed to get physical device");

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        PhysicalDeviceInfo {
            physical_device,
            device_extensions,
        }
    }

    fn create_logical_device(
        info: &PhysicalDeviceInfo,
        event_loop: &ActiveEventLoop,
    ) -> (Arc<Device>, Arc<Queue>) {
        let queue_family_index = info
            .physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(i, q)| {
                q.queue_flags.contains(QueueFlags::GRAPHICS)
                    && info
                        .physical_device
                        .presentation_support(i as u32, event_loop)
                        .unwrap()
            })
            .expect("cannot find queue family with graphics and compute capabilities");

        let (device, mut queues) = Device::new(
            info.physical_device.clone(),
            DeviceCreateInfo {
                enabled_extensions: info.device_extensions,
                enabled_features: DeviceFeatures {
                    dynamic_rendering: true,
                    ..Default::default()
                },
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: queue_family_index as u32,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();

        (device, queues.next().unwrap())
    }

    /// Returns the instance.
    #[inline]
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    /// Returns the device.
    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    /// Returns the graphics queue.
    #[inline]
    pub fn graphics_queue(&self) -> &Arc<Queue> {
        &self.graphics_queue
    }

    /// Returns the memory allocator.
    #[inline]
    pub fn memory_allocator(&self) -> &Arc<StandardMemoryAllocator> {
        &self.memory_allocator
    }

    #[inline]
    pub fn command_allocator(&self) -> &Arc<StandardCommandBufferAllocator> {
        &self.command_buffer_allocator
    }
}
