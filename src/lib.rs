

pub mod application;
pub mod camera;
pub mod formats;
pub mod model;
pub mod modes;
pub mod shaders;
pub mod state;

// TODO: upgrade winit
//  because of shared dependencies, this requires egui, glium and egui_glium
//  to either use newer versions or be ported to use newer winit

pub mod config {
    pub const TITLE: &str = "Phobia";
}

pub mod logging;

pub mod prelude;

#[cfg(debug_assertions)]
pub(crate) mod dev {
    pub const QUICK_MODEL: &str = "models/twocubes_blender.obj";
}

// TODO: theme colors
pub mod theme {}
