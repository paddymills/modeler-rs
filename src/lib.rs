

mod application;
mod state;
pub use application::Application;
pub use state::State;

pub mod camera;
pub mod formats;
pub mod logging;
pub mod prelude;
pub mod model;
pub mod env;
pub mod shaders;

// TODO: upgrade winit
//  because of shared dependencies, this requires egui, glium and egui_glium
//  to either use newer versions or be ported to use newer winit

pub mod config {
    pub const TITLE: &str = "Phobia";
}


#[cfg(debug_assertions)]
pub(crate) mod dev {
    pub const QUICK_MODEL: &str = "models/twocubes_blender.obj";
}

// TODO: theme colors
pub mod theme {}
