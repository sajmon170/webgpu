mod gpu;
mod data;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::gpu::Gpu;

#[derive(Default)]
struct App {
    gpu: Option<Gpu>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = PhysicalSize::new(640, 480);

        let attrs = Window::default_attributes()
            .with_inner_size(size.clone())
            .with_resizable(false);

        let window = event_loop.create_window(attrs).unwrap();
        self.gpu = Some(pollster::block_on(Gpu::new(window, size)).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let gpu = if let Some(gpu) = &mut self.gpu {
            gpu
        } else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window.");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                gpu.resize(size);
            }
            WindowEvent::RedrawRequested => {
                gpu.render().unwrap();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::default()).unwrap();
}
