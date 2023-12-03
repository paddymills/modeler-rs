
mod model;
mod entity;
mod block;
mod sketch;

pub use model::Model;
pub use entity::ModelEntity;
pub use block::Block;
pub use sketch::Sketch;


use crate::buffer::VertexBuffer;
use glium::{
    Display,
    glutin::surface::WindowSurface
};

#[derive(Debug)]
pub struct Plane {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug)]
pub struct Point2d {
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub trait ModelEntityObject {
    fn vertex_buffer(&mut self, display: &Display<WindowSurface>) -> &VertexBuffer;
}

