use crate::{
    renderer::swapchain::SwapchainBundle,
    resources::{
        buffers::VertexT,
        shaders::{fs, vs},
    },
};
use std::sync::Arc;

use vulkano::{
    device::Device,
    pipeline::{
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        graphics::{
            GraphicsPipelineCreateInfo,
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            subpass::{PipelineRenderingCreateInfo, PipelineSubpassType},
            vertex_input::{Vertex, VertexDefinition},
            viewport::ViewportState,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    shader::EntryPoint,
};

//TODO: Is this struct really needed?
pub struct PipelineBundle {
    pipeline: Arc<GraphicsPipeline>,
}

impl PipelineBundle {
    pub fn new(device: &Arc<Device>, swapchain_b: &SwapchainBundle) -> PipelineBundle {
        let [vs, fs] = Self::load_shaders(device);

        let vertex_input_state = VertexT::per_vertex().definition(&vs).unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
        .unwrap();

        let rendering_info = PipelineRenderingCreateInfo {
            color_attachment_formats: vec![Some(swapchain_b.image_format())],
            ..Default::default()
        };

        let pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    rendering_info.color_attachment_formats.len() as u32,
                    ColorBlendAttachmentState::default(),
                )),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                // This graphics pipeline object concerns the first pass of the render pass.
                subpass: Some(PipelineSubpassType::BeginRendering(rendering_info)),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap();

        PipelineBundle { pipeline }
    }

    fn load_shaders(device: &Arc<Device>) -> [EntryPoint; 2] {
        [vs::load, fs::load].map(|load| {
            load(device.clone())
                .expect("failed to create shader module")
                .entry_point("main")
                .unwrap()
        })
    }

    pub fn pipeline(&self) -> &Arc<GraphicsPipeline> {
        &self.pipeline
    }
}
