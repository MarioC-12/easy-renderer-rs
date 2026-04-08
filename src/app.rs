use std::sync::Arc;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::renderer::Renderer;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct TriangleApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for TriangleApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Rust Triangle!")
                    .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
                    .with_resizable(false)
                    .with_visible(true),
            )
            .unwrap();

        let window = Arc::new(window);

        self.renderer = Some(Renderer::new(window.clone(), event_loop));
        self.window = Some(window);
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
            WindowEvent::RedrawRequested => {
                self.renderer.as_mut().unwrap().draw_frame();
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
        }
    }
}
