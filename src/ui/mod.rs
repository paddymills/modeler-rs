
pub mod menu;

pub enum UiDrawResult {
    EnterSketcher,
    ExitSketcher(Option<crate::model::ModelEntity>),
    ShowBlockDialog,
}
