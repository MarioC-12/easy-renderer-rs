use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
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
use winit::event_loop::EventLoop;

pub struct VulkanContext {
    _messenger: Option<DebugUtilsMessenger>,
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
}

struct PhysicalDeviceInfo {
    physical_device: Arc<PhysicalDevice>,
    queue_family_index: u32,
    device_extensions: DeviceExtensions,
}

impl VulkanContext {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let (instance, _messenger) = Self::create_instance(event_loop);
        let phys_info = Self::select_physical_device(&instance, event_loop);
        let (device, queue) = Self::create_logical_device(&phys_info);
        let memory_allocator = Arc::new(StandardMemoryAllocator::new(
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

    fn select_physical_device(
        instance: &Arc<Instance>,
        event_loop: &EventLoop<()>,
    ) -> PhysicalDeviceInfo {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags
                            .contains(QueueFlags::GRAPHICS | QueueFlags::COMPUTE)
                            && p.presentation_support(i as u32, event_loop).unwrap()
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        PhysicalDeviceInfo {
            physical_device,
            queue_family_index,
            device_extensions,
        }
    }

    fn create_logical_device(info: &PhysicalDeviceInfo) -> (Arc<Device>, Arc<Queue>) {
        let (device, mut queues) = Device::new(
            info.physical_device.clone(),
            DeviceCreateInfo {
                enabled_extensions: info.device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: info.queue_family_index,
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
}
