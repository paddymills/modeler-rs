
use crate::prelude::*;

use super::{Plane, Point2d};

#[derive(Debug)]
pub struct Sketch {
    plane: Plane,
    points: Vec<Point2d>
}

impl super::ModelEntityObject for Sketch {
    fn vertex_buffer(&self, display: &crate::prelude::Display) -> VertexBuffer {
        let vertices: Vec<Vertex> = self.points
            .iter()
            .map(|p| self.plane.vertex(p))
            .collect();

        VertexBuffer::new(display, &vertices).unwrap()
    }
}
