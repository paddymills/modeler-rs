
use crate::{environment::Sketcher, application::Application};

#[derive(Debug)]
pub struct Modeling {}

impl Modeling {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::ApplicationMode for Modeling {
    fn draw_toolbar(&self, app: &mut Application, ui: &mut egui::Ui) {
        if ui.button("+ Sketch").clicked() {
            app.camera.set_rotation((0.0, 0.0, 0.0));
            app.mode = Box::new(Sketcher::new());
        }
    }

    fn handle_window_event(&mut self, event: winit::event::WindowEvent) {
        
    }
}
