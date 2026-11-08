use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{self, ActiveEventLoop}, window::{self, Window}};

use crate::sim::sim::{self, Simulation};

pub struct App {
    window: Option<Arc<Window>>,
    sim: Option<Simulation>,
    pixels: Option<Pixels<'static>>,
    mouse_pos: (f32, f32),
    is_left_clicked: bool,
    current_tool: crate::sim::element::ElementType,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            sim: None,
            pixels: None,
            mouse_pos: (0.0, 0.0),
            is_left_clicked: false,
            current_tool: crate::sim::element::ElementType::Sand,
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
            WindowEvent::KeyboardInput { event: key_event, ..} => {
                if key_event.state.is_pressed() {
                    if let winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyP) = key_event.physical_key {
                        use crate::sim::element::ElementType;
                        self.current_tool = match self.current_tool {
                            ElementType::Sand => {
                                ElementType::Water
                            },
                            ElementType::Water => {
                                ElementType::Wall
                            },
                            ElementType::Wall => {
                                ElementType::Sand
                            },
                            _ => {
                                ElementType::Empty
                            },
                        };
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    self.is_left_clicked = state.is_pressed();
                }
            }
            WindowEvent::CursorMoved { position , ..} => {
                self.mouse_pos = (position.x as f32, position.y as f32);
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(physical_size.width, physical_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(sim), Some(pixels)) = (&self.window, &mut self.sim, &mut self.pixels) {

                    if self.is_left_clicked {
                        let window_size = window.inner_size();

                        let scale_x = sim.grid.width as f32 / window_size.width as f32;
                        let scale_y = sim.grid.height as f32 / window_size.height as f32;

                        let sim_x = (self.mouse_pos.0 * scale_x) as u32;
                        let sim_y = (self.mouse_pos.1 * scale_y) as u32;

                        sim.spawn_pixel(sim_x, sim_y, self.current_tool);
                    }

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