
use glium::{
    Display,
    glutin::surface::WindowSurface
};
use obj::Obj;

use super::*;
use crate::{formats::wavefront, buffer::Vertex};

#[derive(Debug)]
pub enum ModelEntity {
    ImportedModel(Obj),

    Sketch(Sketch),

    // TODO: add anchor
    Block(Block)
}

impl ModelEntity {
    pub fn vertex_buffer(&self, display: &Display<WindowSurface>) -> glium::VertexBuffer<Vertex> {
        use ModelEntity::*;

        match self {
            ImportedModel(obj) => wavefront::load(display, obj),
            _ => todo!()
        }
    }
}
