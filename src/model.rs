
use std::{
    fs::File,
    path::PathBuf
};

use glium::{
    Display,
    glutin::surface::WindowSurface
};

use obj::{Obj, ObjData};
use crate::formats::wavefront;
use crate::buffer::VertexBuffer;

#[derive(Debug)]
pub struct Model {
    pub geometry: Obj,

    pub vb: Option<VertexBuffer>
}

impl Model {
    pub fn new() -> Self {
        Self {
            geometry: Obj { data: ObjData::default(), path: PathBuf::new() },
            vb: None
        }
    }

    pub fn load(&mut self, path: PathBuf) -> Result<(), obj::ObjError> {
        let file = File::open(&path)?;
        self.geometry.path = path;
        self.geometry.data = ObjData::load_buf(file)?;

        // invalidate cached vertex buffer
        self.vb = None;

        Ok(())
    }

    pub fn load_obj(&mut self, path: &PathBuf) {
        self.geometry = Obj::load(path).unwrap();

        // invalidate cached vertex buffer
        self.vb = None;
    }

    pub fn vertex_buffer(&mut self, display: &Display<WindowSurface>) -> &VertexBuffer {
        if let None = self.vb {
            self.vb = Some( wavefront::load(display, &self.geometry) );
        }

        // previous lines ensure this will not panic
        self.vb.as_ref().unwrap()
    }
}
