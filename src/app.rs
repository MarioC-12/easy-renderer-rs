use std::sync::Arc;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
    renderer::{Renderer, context::VulkanContext},
    resources::{buffers::VertexT, textures::Texture},
    scene::mesh::Mesh,
};

const VERTICES: [VertexT; 24] = [
    // Front face (z = 0.5)
    VertexT {
        in_position: [-0.5, -0.5, 0.5],
        in_color: [1.0, 0.0, 0.0],
        tex_coord: [0.0, 0.0],
    },
    VertexT {
        in_position: [0.5, -0.5, 0.5],
        in_color: [1.0, 0.0, 0.0],
        tex_coord: [1.0, 0.0],
    },
    VertexT {
        in_position: [0.5, 0.5, 0.5],
        in_color: [1.0, 0.0, 0.0],
        tex_coord: [1.0, 1.0],
    },
    VertexT {
        in_position: [-0.5, 0.5, 0.5],
        in_color: [1.0, 0.0, 0.0],
        tex_coord: [0.0, 1.0],
    },
    // Back face (z = -0.5)
    VertexT {
        in_position: [0.5, -0.5, -0.5],
        in_color: [0.0, 1.0, 0.0],
        tex_coord: [0.0, 0.0],
    },
    VertexT {
        in_position: [-0.5, -0.5, -0.5],
        in_color: [0.0, 1.0, 0.0],
        tex_coord: [1.0, 0.0],
    },
    VertexT {
        in_position: [-0.5, 0.5, -0.5],
        in_color: [0.0, 1.0, 0.0],
        tex_coord: [1.0, 1.0],
    },
    VertexT {
        in_position: [0.5, 0.5, -0.5],
        in_color: [0.0, 1.0, 0.0],
        tex_coord: [0.0, 1.0],
    },
    // Right face (x = 0.5)
    VertexT {
        in_position: [0.5, -0.5, 0.5],
        in_color: [0.0, 0.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    VertexT {
        in_position: [0.5, -0.5, -0.5],
        in_color: [0.0, 0.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    VertexT {
        in_position: [0.5, 0.5, -0.5],
        in_color: [0.0, 0.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    VertexT {
        in_position: [0.5, 0.5, 0.5],
        in_color: [0.0, 0.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    // Left face (x = -0.5)
    VertexT {
        in_position: [-0.5, -0.5, -0.5],
        in_color: [1.0, 1.0, 0.0],
        tex_coord: [0.0, 0.0],
    },
    VertexT {
        in_position: [-0.5, -0.5, 0.5],
        in_color: [1.0, 1.0, 0.0],
        tex_coord: [1.0, 0.0],
    },
    VertexT {
        in_position: [-0.5, 0.5, 0.5],
        in_color: [1.0, 1.0, 0.0],
        tex_coord: [1.0, 1.0],
    },
    VertexT {
        in_position: [-0.5, 0.5, -0.5],
        in_color: [1.0, 1.0, 0.0],
        tex_coord: [0.0, 1.0],
    },
    // Top face (y = 0.5)
    VertexT {
        in_position: [-0.5, 0.5, 0.5],
        in_color: [0.0, 1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    VertexT {
        in_position: [0.5, 0.5, 0.5],
        in_color: [0.0, 1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    VertexT {
        in_position: [0.5, 0.5, -0.5],
        in_color: [0.0, 1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    VertexT {
        in_position: [-0.5, 0.5, -0.5],
        in_color: [0.0, 1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    // Bottom face (y = -0.5)
    VertexT {
        in_position: [-0.5, -0.5, -0.5],
        in_color: [1.0, 0.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    VertexT {
        in_position: [0.5, -0.5, -0.5],
        in_color: [1.0, 0.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    VertexT {
        in_position: [0.5, -0.5, 0.5],
        in_color: [1.0, 0.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    VertexT {
        in_position: [-0.5, -0.5, 0.5],
        in_color: [1.0, 0.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
];

const INDEXES: [u32; 36] = [
    0, 2, 1, 2, 0, 3, // Front
    4, 6, 5, 6, 4, 7, // Back
    8, 10, 9, 10, 8, 11, // Right
    12, 14, 13, 14, 12, 15, // Left
    16, 18, 17, 18, 16, 19, // Top
    20, 22, 21, 22, 20, 23, // Bottom
];

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct TriangleApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    mesh: Option<Mesh>,
    suspended: bool,
}

impl ApplicationHandler for TriangleApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Rust Cube!")
                    .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
                    .with_resizable(true)
                    .with_visible(true),
            )
            .unwrap();

        let window = Arc::new(window);
        let context = Arc::new(VulkanContext::new(event_loop));
        let mesh = Mesh::new(
            context.memory_allocator(),
            context.command_allocator(),
            context.graphics_queue(),
            &VERTICES,
            &INDEXES,
        );
        let texture = Texture::load_texture(
            context.memory_allocator(),
            context.command_allocator(),
            context.graphics_queue(),
            "textures/texture.jpg",
            context.texture_sampler(),
        )
        .unwrap();

        let rend = Renderer::new(context.clone(), window.clone(), &texture);

        self.renderer = Some(rend);
        self.window = Some(window);
        self.mesh = Some(mesh);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Stopping program!");
                event_loop.exit();
            }
            WindowEvent::Occluded(occluded) => {
                self.suspended = occluded;
                if !occluded {
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            WindowEvent::Resized(_) => {
                self.renderer.as_mut().unwrap().handle_resize();
            }
            WindowEvent::RedrawRequested => {
                if !self.suspended {
                    if let Err(e) = self
                        .renderer
                        .as_mut()
                        .unwrap()
                        .draw_frame(self.mesh.as_ref().unwrap())
                    {
                        eprint!("Frame skipped: {e}");
                    }
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            _ => (),
        }
    }
}

impl TriangleApp {
    pub fn new() -> Self {
        TriangleApp {
            window: None,
            renderer: None,
            mesh: None,
            suspended: false,
        }
    }
}
