use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{self, ActiveEventLoop}, window::{self, Window}};

use crate::sim::sim::{self, Simulation};

pub struct App {
    window: Option<Arc<Window>>,
    sim: Option<Simulation>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            sim: None,
            pixels: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = Window::default_attributes()
                .with_title("Machina Sim")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
            self.window = Some(window.clone());

            let sim_width = 320;
            let sim_height = 180;
            self.sim = Some(Simulation::new(sim_width, sim_height));

            let physical_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(physical_size.width, physical_size.height, window);

            println!("Initialising hardware graphids.");
            let pixels = Pixels::new(sim_width, sim_height, surface_texture)
                .expect("Failed to initialise GPU buffer.");

            self.pixels = Some(pixels);
            println!("Hardware initialised!");
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested, exiting.");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(physical_size.width, physical_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(sim), Some(pixels)) = (&self.window, &mut self.sim, &mut self.pixels) {
                    sim.step();

                    let frame = pixels.frame_mut();
                    frame.copy_from_slice(&sim.raw_output_buffer);

                    if let Err(err) = pixels.render() {
                        println!("Graphics rendering error: {:?}", err);
                        event_loop.exit();
                        return;
                    }

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