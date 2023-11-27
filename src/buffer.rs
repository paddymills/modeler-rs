

pub type VertexBuffer = glium::vertex::VertexBuffer<Vertex>;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture: [f32; 2],
}
