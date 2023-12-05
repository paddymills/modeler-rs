
mod modeling;
mod sketcher;

pub use modeling::Modeling;
pub use sketcher::Sketcher;

use crate::application::Application;

pub trait ApplicationMode
    where Self: std::fmt::Debug
{
    fn draw_toolbar(&self, app: &mut Application, ui: &mut egui::Ui);
    fn handle_window_event(&mut self, event: winit::event::WindowEvent);
}
