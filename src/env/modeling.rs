
// use super::Sketcher;

#[derive(Debug)]
pub struct Modeling {}

impl Modeling {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::ApplicationEnvironment for Modeling {
    fn draw_toolbar(&self, ui: &mut egui::Ui) {
        if ui.button("+ Sketch").clicked() {
            log::trace!("Add sketch selected")
            // app.camera.set_rotation((0.0, 0.0, 0.0));
            // app.mode = Box::new(Sketcher::new());
        }
    }

    fn handle_window_event(&mut self, event: winit::event::WindowEvent) {
        
    }
}
