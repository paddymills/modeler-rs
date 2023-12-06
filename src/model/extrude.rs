use crate::prelude::*;


#[derive(Debug)]
pub struct Extrude {
    pub sketch: super::Sketch,

    // dir: Point3d,

    pub dist: f32
}

impl Extrude {
    pub fn points(&self) -> Vec<Point3d> {
        let mut points = Vec::new();

        for mut point in self.sketch.points() {
            points.push( point.clone() );

            point.z = self.dist;
            points.push( point.clone() );
        }

        points
    }
}

impl super::ModelEntityObject for Extrude {
    fn vertices(&self) -> Vec<crate::prelude::Vertex> {
        
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