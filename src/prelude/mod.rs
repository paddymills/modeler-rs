
pub type Display = glium::Display<glium::glutin::surface::WindowSurface>;

mod point;
mod vertex;

pub use point::{Point2d, Point3d};
pub use vertex::{Vertex, VertexBuffer};

/// register components with opengl compatibility
pub fn register() {
    glium::implement_vertex!(Vertex, position, normal, texture);
}
