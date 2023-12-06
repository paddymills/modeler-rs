
use crate::prelude::*;

use super::{Plane, Point2d};

#[derive(Debug, Default)]
pub struct Sketch {
    plane: Plane,
    points: Vec<Point2d>
}

impl Sketch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_points(points: Vec<Point2d>) -> Self {
        Self { points, ..Default::default() }
    }
}

impl super::ModelEntityObject for Sketch {
    fn vertices(&self) -> Vec<Vertex> {
        self.points
            .iter()
            .map(|p| self.plane.vertex(p))
            .collect()
    }
}
