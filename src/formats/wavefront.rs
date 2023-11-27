
use glium::{
    implement_vertex,
    Display,
    glutin::surface::WindowSurface,
    vertex::VertexBuffer,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load(display: &Display<WindowSurface>, data: &obj::Obj) -> glium::vertex::VertexBufferAny {

    implement_vertex!(Vertex, position, normal, texture);
    
    let mut vertex_data = Vec::new();
    
    let data = &data.data;
    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position,
                            normal,
                            texture,
                        })
                    }
                },
            }
        }
    }

    VertexBuffer::new(display, &vertex_data).unwrap().into()
}
