
use super::Point3d;
use crate::prelude::*;

#[derive(Debug)]
pub struct Block {
    origin: Point3d,
    dim: Point3d
}

impl Block {
    pub fn points(&self) -> Vec<Point3d> {
        let min = self.origin.clone();
        let max = min + self.dim;

        vec![
            // bottom
            min,
            Point3d { x: min.x, y: max.y, z: min.z },
            Point3d { x: max.x, y: min.y, z: min.z },
            Point3d { x: max.x, y: max.y, z: min.z },

            // top
            Point3d { x: min.x, y: min.y, z: max.z },
            Point3d { x: max.x, y: min.y, z: max.z },
            Point3d { x: min.x, y: max.y, z: max.z },
            max
        ]
    }
}

impl super::ModelEntityObject for Block {
    fn vertex_buffer(&mut self, display: &crate::prelude::Display) -> VertexBuffer {
        let vertices: Vec<Vertex> = self.points().into_iter()
            .map(Into::into)
            .collect();

        VertexBuffer::new(display, &vertices).unwrap()
    }
}
