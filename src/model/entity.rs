
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
    pub fn vertices(&self) -> Vec<Vertex> {
        use ModelEntity::*;

        match self {
            ImportedModel(obj) => wavefront::load(obj),
            Block(bl) => bl.vertices(),
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
