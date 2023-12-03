
use super::{Plane, Point2d};

#[derive(Debug)]
pub struct Sketch {
    plane: Plane,
    points: Vec<Point2d>
}