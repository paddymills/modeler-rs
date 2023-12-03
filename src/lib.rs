

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

pub mod logging;

pub mod prelude {
    pub type VertexBuffer = glium::vertex::VertexBuffer<Vertex>;
    pub type Display = glium::Display<glium::glutin::surface::WindowSurface>;

    #[derive(Debug, Copy, Clone)]
    pub struct Vertex {
        pub position: [f32; 3],
        pub normal: [f32; 3],
        pub texture: [f32; 2],
    }
}

// TODO: theme colors
pub mod theme {}
