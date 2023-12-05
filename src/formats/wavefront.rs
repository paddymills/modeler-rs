
use crate::prelude::*;


// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load(data: &obj::Obj) -> Vec<Vertex> {
    load_data(&data.data)
}

pub fn load_data(data: &obj::ObjData) -> Vec<Vertex> {
    let mut vertex_data = Vec::new();

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

                        let vertex = Vertex {
                            position,
                            normal,
                            texture,
                        };

                        log::debug!("adding {:?}", vertex);
                        vertex_data.push(vertex)
                    }
                },
            }
        }
    }

    vertex_data
}
