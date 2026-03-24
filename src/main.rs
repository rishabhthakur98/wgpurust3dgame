// src/main.rs
mod camera;
mod control;
mod core;
mod floor;
mod player;
mod render;
mod sky;

use core::GameState;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut game = GameState::new();
    let _ = event_loop.run_app(&mut game);
}
