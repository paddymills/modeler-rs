
use glium::{
    Display,
    glutin::surface::WindowSurface
};
use obj::Obj;

use super::*;
use crate::{prelude::*, formats::wavefront};

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
            Block(bl) => bl.vertex_buffer(display),
            _ => todo!()
        }
    }
}

impl ToString for ModelEntity {
    
    fn to_string(&self) -> String {
        use ModelEntity::*;

        match self {
            ImportedModel(_) => "ImportedModel",
            Sketch(_) => "Sketch",
            Block(_) => "Block",
        }.into()
    }
}
