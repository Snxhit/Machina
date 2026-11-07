use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{self, ActiveEventLoop}, window::{self, Window}};

use crate::{gui::Gui, sim::{element::ElementType, sim::{self, Simulation}}};

pub struct App {
    window: Option<Arc<Window>>,
    sim: Option<Simulation>,
    pixels: Option<Pixels<'static>>,
    gui: Option<Gui>,
    selected_element: ElementType,
    mouse_pos: (f32, f32),
    is_left_clicked: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            sim: None,
            pixels: None,
            gui: None,
            selected_element: ElementType::Sand,
            mouse_pos: (0.0, 0.0),
            is_left_clicked: false,
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
            let surface_texture = SurfaceTexture::new(physical_size.width, physical_size.height, window.clone());

            println!("Initialising hardware graphids.");
            let pixels = Pixels::new(sim_width, sim_height, surface_texture)
                .expect("Failed to initialise GPU buffer.");

            self.gui = Some(Gui::new(&window, pixels.device(), pixels.surface_texture_format()));

            self.pixels = Some(pixels);
            println!("Hardware initialised!");
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
        let Some(window) = self.window.clone() else {
            return;
        };

        let mut egui_consumed = false;
        if let Some(gui) = self.gui.as_mut() {
            egui_consumed = gui.on_window_event(&window, &event);
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested, exiting.");
                event_loop.exit();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    if egui_consumed {
                        self.is_left_clicked = false;
                    } else {
                        self.is_left_clicked = state.is_pressed();
                    }
                }
            }
            WindowEvent::CursorMoved { position , ..} => {
                if !egui_consumed {
                    self.mouse_pos = (position.x as f32, position.y as f32);
                }
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(physical_size.width, physical_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(sim) = self.sim.as_mut() else {
                    return;
                };

                let Some(gui) = self.gui.as_mut() else {
                    return;
                };

                let full_output = gui.update(&window, &mut self.selected_element);

                if self.is_left_clicked && !gui.wants_pointer_input() {
                    let window_size = window.inner_size();

                    let scale_x = sim.grid.width as f32 / window_size.width as f32;
                    let scale_y = sim.grid.height as f32 / window_size.height as f32;

                    let sim_x = (self.mouse_pos.0 * scale_x) as u32;
                    let sim_y = (self.mouse_pos.1 * scale_y) as u32;

                    sim.spawn_pixel(sim_x, sim_y, self.selected_element);
                }

                sim.step();

                let Some(pixels) = self.pixels.as_mut() else {
                    return;
                };
                pixels.frame_mut().copy_from_slice(&sim.raw_output_buffer);

                let pixels = self.pixels.as_ref().unwrap();
                if let Err(err) = pixels.render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);
                    gui.paint(&window, encoder, render_target, context, full_output);
                    Ok(())
                }) {
                    println!("Graphics rendering error: {:?}", err);
                    event_loop.exit();
                    return;
                }

                window.pre_present_notify();
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