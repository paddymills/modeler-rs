
use std::{
    fs::File,
    path::PathBuf
};

use glium::{
    Display,
    glutin::surface::WindowSurface
};

use obj::{Obj, ObjData};
use crate::buffer::VertexBuffer;
use super::ModelEntity;


#[derive(Debug, Default)]
pub struct Model {
    geometry: Vec<ModelEntity>,
    vb: Option<VertexBuffer>
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save(&mut self, path: &PathBuf) -> Result<(), obj::ObjError> {
        match &self.geometry[0] {
            ModelEntity::ImportedModel(obj) => obj.save(path),
            _ => todo!()
        }
    }

    pub fn load(&mut self, path: PathBuf) -> Result<(), obj::ObjError> {
        let file = File::open(&path)?;
        self.geometry = vec![ModelEntity::ImportedModel(Obj { data: ObjData::load_buf(file)?, path })];

        // invalidate cached vertex buffer
        self.vb = None;

        Ok(())
    }

    pub fn load_obj(&mut self, path: &PathBuf) {
        self.geometry = vec![ModelEntity::ImportedModel(Obj::load(path).unwrap())];

        // invalidate cached vertex buffer
        self.vb = None;
    }

    pub fn vertex_buffer(&mut self, display: &Display<WindowSurface>) -> &VertexBuffer {
        if let None = self.vb {
            self.vb = match self.geometry.len() {
                0 => Some(crate::buffer::empty_buffer(display)),

                // TODO: impl for multiple geometry entities
                _ => Some( self.geometry[0].vertex_buffer(display) )
            };
        }

        // previous lines ensure this will not panic
        self.vb.as_ref().unwrap()
    }
}
