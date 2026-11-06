use std::sync::Arc;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{self, ActiveEventLoop}, window::Window};


pub struct App {
    window: Option<Arc<Window>>,
}

impl App {
    pub fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = Window::default_attributes()
                .with_title("Machina Sim")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
            self.window = Some(window);
            println!("Window initialised.");
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested, exiting.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.pre_present_notify();
                }
            }
            _ => {}, 
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}