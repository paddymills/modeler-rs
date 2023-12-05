

use crate::ui::UiDrawResult;

use super::{ApplicationEnvironmentOps, ApplicationEnvironmentType};

#[derive(Debug)]
pub struct Modeler {
    pub camera: super::Camera
}

impl Modeler {
    pub fn new(camera: super::Camera) -> Self {
        Self { camera }
    }
}

impl ApplicationEnvironmentOps for Modeler {
    fn draw_toolbar(&mut self, ui: &mut egui::Ui) -> Option<UiDrawResult> {
        if ui.button("+ Sketch").clicked() {
            log::trace!("Add sketch selected");
            self.camera.lock().unwrap().set_rotation((0.0, 0.0, 0.0));
            
            return Some(UiDrawResult::EnterSketcher);
        }

        if ui.button("+ Block").clicked() {
            log::trace!("Add block selected");
            self.camera.lock().unwrap().set_rotation((0.0, 0.0, 0.0));
            
            return Some(UiDrawResult::ShowBlockDialog);
        }

        None
    }

    fn handle_window_event(&mut self, event: &winit::event::WindowEvent) {
        match self.camera.lock() {
            Ok(mut camera) => camera.process_input(&event),
            Err(e) => log::error!("Failed to lock camera to handle WindowEvent<{:?}> because `{}`", event, e)
        }
    }
}

impl From<&ApplicationEnvironmentType> for Modeler {
    fn from(env: &ApplicationEnvironmentType) -> Self {
        match env {
            ApplicationEnvironmentType::Sketching(sketcher) => Self { camera: sketcher.camera.clone()},
            _ => unimplemented!()
        }
    }
}
