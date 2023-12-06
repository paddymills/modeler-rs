
use std::ops::{Deref, DerefMut};

use crate::prelude::Vertex;
use super::{Point2d, Point3d};


#[derive(Debug, Default, Clone)]
pub struct Plane(Point3d);

impl Plane {
    /// create a 3d point depending on the normal direction of the plane
    pub fn point(&self, point: &Point2d) -> Point3d {
        // point is assumed to be (x, y) cordinates local to the orientation of the plane

        // depending on the orientation of the plane, (x, y) transorms into its part in (x, y, z)
        // note that point.x might not be result.x
        // i.e. the plane the global YZ plane, so (x, y) -> (0, x, y)

        // TODO: implement and replace this placeholder
        Point3d { x: point.x, y: point.y, z: self.z }
    }

    /// create [`Vertex`] from a given point on the plane
    pub fn vertex(&self, point: &Point2d) -> Vertex {
        Vertex {
            position: self.point(point).to_array(),
            normal: self.to_array(),
            ..Default::default()
        }
    }
}

impl Deref for Plane {
    type Target = Point3d;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Plane {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
