
use winit::event::{WindowEvent, ElementState, MouseButton};

use crate::{prelude::*, model::{ModelEntity, Sketch}, ui::UiDrawResult};
use super::{ApplicationEnvironmentOps, ApplicationEnvironmentType};

#[derive(Debug, Default)]
pub struct Sketcher {
    pub camera: super::Camera,

    mouse_pos: Point2d,
    points: Vec<Point2d>
}

impl Sketcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finalize(&self) {
        log::error!("finalize not implemented")
    }
}

impl ApplicationEnvironmentOps for Sketcher {
    fn draw_toolbar(&mut self, ui: &mut egui::Ui) -> Option<UiDrawResult> {
        if ui.button("Finish sketch (right-click)").clicked() {
            self.finalize();

            return Some(UiDrawResult::ExitSketcher(Some(
                ModelEntity::Sketch(Sketch::with_points(self.points.clone()))
            )));
        }

        None
    }

    fn handle_window_event(&mut self, event: &WindowEvent) {
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
                    MouseButton::Right => (),
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

impl From<&ApplicationEnvironmentType> for Sketcher {
    fn from(env: &ApplicationEnvironmentType) -> Self {
        match env {
            ApplicationEnvironmentType::Modeling(modeler) => Self { camera: modeler.camera.clone(), ..Default::default() },
            _ => unimplemented!()
        }
    }
}