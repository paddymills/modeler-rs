

pub mod application;
pub mod buffer;
pub mod formats;
pub mod model;
pub mod shaders;
pub mod support;

// TODO: upgrade winit
//  because of shared dependencies, this requires egui, glium and egui_glium
//  to either use newer versions or be ported to use newer winit

pub mod config {
    pub const TITLE: &str = "Phobia";
}

// TODO: prelude for crate level types (i.e., Display)
pub mod prelude {}

// TODO: theme colors
pub mod theme {}
