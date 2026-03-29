mod app;
mod renderer;

use winit::{
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop},
};

use crate::app::TriangleApp;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = TriangleApp::new(&event_loop);

    event_loop.run_app(&mut app)?;
    Ok(())
}
