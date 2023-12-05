use std::ops::Add;

use super::Vertex;


pub type Point2d = winit::dpi::PhysicalPosition<f32>;
// #[derive(Debug, Default, Clone, Copy)]
// pub struct Point2d {
//     pub x: f32,
//     pub y: f32
// }

// impl Point2d {
//     pub fn to_array(&self) -> [f32; 2] {
//         [self.x, self.y]
//     }
// }

// impl From<PhysicalSize<f64>> for Point2d {
//     fn from(value: PhysicalSize<f64>) -> Self {
//         Self { x: value.width as f32, y: value.height as f32 }
//     }
// }

// impl From<PhysicalPosition<f64>> for Point2d {
//     fn from(value: PhysicalPosition<f64>) -> Self {
//         Self { x: value.x as f32, y: value.y as f32 }
//     }
// }

#[derive(Debug, Default, Clone, Copy)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point3d {
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Add for Point3d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Into<Vertex> for Point3d {
    fn into(self) -> Vertex {
        Vertex { position: self.to_array(), ..Default::default() }
    }
}
