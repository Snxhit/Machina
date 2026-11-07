mod app;
mod sim;
mod gui;

use app::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    println!("Launching Machina event loop...");
    event_loop.run_app(&mut app).unwrap();
}