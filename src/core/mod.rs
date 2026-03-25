pub mod config;

use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window, WindowId};

use crate::camera::Camera;
use crate::control::InputState;
use crate::player::Player;
use crate::render::Renderer;
use crate::world::WorldState; // NEW: Import WorldState

pub struct GameState<'a> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer<'a>>,
    last_frame_time: Instant,
    
    player: Player,
    camera: Camera,
    input: InputState,
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            last_frame_time: Instant::now(),
            player: Player::new(),
            camera: Camera::new(),
            input: InputState::new(),
        }
    }
}

impl<'a> ApplicationHandler for GameState<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Rust AAA Engine Structure")
                .with_inner_size(winit::dpi::LogicalSize::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT));
            
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let _ = window.set_cursor_grab(CursorGrabMode::Confined).or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked));
            window.set_cursor_visible(false);

            self.renderer = Some(Renderer::new(window.clone()));
            self.window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => if let Some(r) = &mut self.renderer { r.resize(physical_size) },
            WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, .. }, .. } => {
                if physical_key == PhysicalKey::Code(KeyCode::Escape) { event_loop.exit(); }
                self.input.process_keyboard(physical_key, state == ElementState::Pressed);
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now.duration_since(self.last_frame_time).as_secs_f32();
                let frame_time = 1.0 / config::TARGET_FPS as f32;
                
                if dt < frame_time {
                    std::thread::sleep(Duration::from_secs_f32(frame_time - dt));
                    self.window.as_ref().unwrap().request_redraw();
                    return; 
                }
                self.last_frame_time = Instant::now();
                
                // Get world constraints via the new WorldState system
                let world_state = WorldState::new();
                let colliders = world_state.get_colliders();
                
                // Keep player within a 1000x1000 radius of origin
                let limit_x = 1000.0; 
                let limit_z = 1000.0;

                // 1. Move the Player
                self.player.update(dt, self.input.dir, self.camera.yaw, self.input.is_free_look, limit_x, limit_z, &colliders);
                
                // 2. Adjust Camera Distance smoothly using dt
                self.camera.update(dt, self.player.pos, &colliders);
                
                // 3. Render
                if let Some(renderer) = &mut self.renderer {
                    renderer.update_matrices(self.player.pos, self.player.yaw, self.camera.yaw, self.camera.pitch, self.camera.distance, self.input.is_day);
                    let _ = renderer.render(self.input.is_day);
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, _device_id: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.camera.process_mouse(delta.0 as f32, delta.1 as f32);
        }
    }
}