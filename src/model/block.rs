
use super::Point3d;
use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Block {
    origin: Point3d,
    dim: Point3d
}

impl Block {
    pub fn two_points(p1: Point3d, p2: Point3d) -> Self {
        Self { origin: p1, dim: p2 }
    }

    pub fn origin_and_max(dim: Point3d) -> Self {
        Self { dim, ..Default::default() }
    }

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
    fn vertices(&self) -> Vec<Vertex> {
        use itertools::Itertools;

        log::debug!("calculating buffer for block");

        let mut data = Vec::new();
        let points = self.points();
        let permutations = (0..points.len()).permutations(3);

        for point_set in permutations {
            let normal: Point3d = point_set
                .clone().into_iter()
                .map(|p| points[p])
                .reduce(|acc, e| acc + e)
                .unwrap() / 3f32;

            let normal = normal.to_array();
            log::debug!("normal: {:?}", normal);

            for p in point_set {
                data.push(Vertex { position: points[p].to_array(), normal, ..Default::default() });
            }
        }

        data
    }
}
