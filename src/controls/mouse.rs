use std::sync::mpsc::Sender;

use glutin::{
    event::{ElementState, WindowEvent, MouseButton},
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window
};


#[derive(Debug)]
struct Buttons {
    left: ElementState,
    middle: ElementState,
    right: ElementState
}

impl Default for Buttons {
    fn default() -> Self {
        Self {
            left:  ElementState::Released,
            middle: ElementState::Released,
            right:  ElementState::Released
        }
    }
}

// #[derive(Debug)]
pub struct MouseController {
    position: [f32; 2],
    buttons: Buttons,
    tx: Sender<(f32, f32)>
}

impl MouseController {
    pub fn new(tx: Sender<(f32, f32)>) -> Self {
        Self {
            position: [0., 0.],
            buttons: Buttons::default(),
            tx
        }
    }

    pub fn handle_event(&mut self, event: WindowEvent, window: &Window) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.update_position(position, window.inner_size())
            },
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left, ..
            } => {
                let _ = self.tx.send( (self.position[0], self.position[1]) );
                window.request_redraw();
            },
            _ => ()
        }
    }

    pub fn update_position(&mut self, pos: PhysicalPosition<f64>, window_size: PhysicalSize<u32>) {
        let x = -1. + 2. * pos.x as f32 / window_size.width  as f32;
        let y =  1. - 2. * pos.y as f32 / window_size.height as f32;

        self.position = [x, y];
    }

    pub fn position(&self) -> [f32; 2] {
        self.position
    }
}
