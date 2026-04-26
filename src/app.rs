use std::sync::Arc;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{renderer::Renderer, resources::buffers::VertexT, scene::mesh::Mesh};

const VERTICES: [VertexT; 4] = [
    VertexT {
        in_position: [-0.5, -0.5],
        in_color: [1.0, 0.0, 0.0],
    },
    VertexT {
        in_position: [0.5, -0.5],
        in_color: [0.0, 1.0, 0.0],
    },
    VertexT {
        in_position: [0.5, 0.5],
        in_color: [0.0, 0.0, 1.0],
    },
    VertexT {
        in_position: [-0.5, 0.5],
        in_color: [1.0, 1.0, 1.0],
    },
];

const INDEXES: [u32; 6] = [0, 1, 2, 2, 3, 0];

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct TriangleApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    mesh: Option<Mesh>,
}

impl ApplicationHandler for TriangleApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Rust Triangle!")
                    .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
                    .with_resizable(true)
                    .with_visible(true),
            )
            .unwrap();

        let window = Arc::new(window);
        let rend = Renderer::new(window.clone(), event_loop);
        let mesh = Mesh::new(
            rend.context().memory_allocator(),
            rend.context().command_allocator(),
            rend.context().graphics_queue(),
            &VERTICES,
            &INDEXES,
        );

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
            WindowEvent::Resized(_) => {
                self.renderer.as_mut().unwrap().handle_resize();
            }
            WindowEvent::RedrawRequested => {
                self.renderer
                    .as_mut()
                    .unwrap()
                    .draw_frame(self.mesh.as_ref().unwrap());
                self.window.as_ref().unwrap().request_redraw();
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
        }
    }
}
