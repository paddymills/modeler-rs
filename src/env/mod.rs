
mod modeling;
mod sketcher;

pub use modeling::Modeling;
pub use sketcher::Sketcher;


pub trait ApplicationEnvironment
    where Self: std::fmt::Debug
{
    fn draw_toolbar(&self, ui: &mut egui::Ui);
    fn handle_window_event(&mut self, event: winit::event::WindowEvent);
}
