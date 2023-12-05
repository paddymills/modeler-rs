use winit::event::{WindowEvent, ElementState, MouseButton};

use crate::{prelude::*, application::Application};

#[derive(Debug, Default)]
pub struct Sketcher {
    mouse_pos: Point2d,
    points: Vec<Point2d>
}

impl Sketcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finalize(&self) {
        todo!("finalize")
    }
}

impl super::ApplicationMode for Sketcher {
    fn draw_toolbar(&self, app: &mut Application, ui: &mut egui::Ui) {
        if ui.button("Finish sketch (right-click)").clicked() {
            self.finalize();
        }
    }

    fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => self.mouse_pos = (position.x as f32, position.y as f32).into(),
            WindowEvent::MouseInput { state: ElementState::Released, button, .. } => {
                match button {
                    MouseButton::Left => {
                        if self.points.contains(&self.mouse_pos) {
                            log::warn!("Possible duplicate point in sketch since mouse did not move");
                        }
                        
                        else { self.points.push(self.mouse_pos.clone()) }
                    },
                    MouseButton::Middle => (),
                    MouseButton::Right => self.finalize(),
                    _ => ()
                }
            },
            _ => ()
        }
    }
}
