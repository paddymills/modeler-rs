
mod model;
mod entity;

mod block;
mod plane;
mod sketch;

pub use model::Model;
pub use entity::ModelEntity;
pub use block::Block;
pub use plane::Plane;
pub use sketch::Sketch;


use crate::prelude::*;

pub trait ModelEntityObject {
    fn vertex_buffer(&mut self, display: &Display) -> VertexBuffer;
}

