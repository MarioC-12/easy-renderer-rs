use std::sync::Arc;

use vulkano::instance::Instance;

pub struct VulkanContext {
    pub instance: Arc<Instance>,
}

impl VulkanContext {
    pub fn new() {
        VulkanContext {
            instance: Self::create_instance(),
        }
    }

    fn create_instance() {}
}
