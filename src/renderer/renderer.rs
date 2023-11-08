use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::vertex_array::VertexArray;
use crate::set_attribute;

const VERTEX_SHADER_SOURCE: &str = include_str!("shaders/vertex.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("shaders/fragment.glsl");

type Pos = [f32; 2];
type Color = [f32; 3];

#[repr(C, packed)]
struct Vertex(Pos, Color);

// #[rustfmt::skip]
// const VERTICES: [Vertex; 5] = [
//     Vertex([-0.5, -0.5], [1.0, 0.0, 0.0]),
//     Vertex([0.5,  -0.5], [0.0, 1.0, 0.0]),
//     Vertex([0.5,   0.5], [0.0, 0.0, 1.0]),
//     Vertex([0.0,   0.75], [0.0, 0.0, 1.0]),
//     Vertex([-0.5,  0.5], [0.0, 0.0, 1.0]),
// ];

pub struct Renderer {
    program: ShaderProgram,
    _vertex_buffer: Buffer,
    vertex_array: VertexArray,

    vertices: Vec<Vertex>
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        let mut  vertices = Vec::new();

        vertices.push(Vertex([-0.5, -0.5], [1.0, 0.0, 0.0]));
        vertices.push(Vertex([0.5,  -0.5], [0.0, 1.0, 0.0]));
        vertices.push(Vertex([0.5,   0.5], [0.0, 0.0, 1.0]));
        vertices.push(Vertex([0.0,   0.75], [0.0, 0.0, 1.0]));
        vertices.push(Vertex([-0.5,  0.5], [0.0, 0.0, 1.0]));

        unsafe {
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            // vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);
            vertex_buffer.set_data(&vertices, gl::DYNAMIC_DRAW);
            let vertex_array = VertexArray::new();
            let pos_attrib = program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, Vertex::0);
            let color_attrib = program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, Vertex::1);

            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                vertex_array,

                vertices
            })
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        self.vertices.push(Vertex([x, y], [0.0, 0.0, 1.0]));
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self._vertex_buffer.set_data(&self.vertices, gl::DYNAMIC_DRAW);

            self.program.apply();
            self.vertex_array.bind();
            gl::DrawArrays(gl::LINE_LOOP, 0, self.vertices.len() as i32);
            // gl::DrawArrays(gl::TRIANGLE_STRIP, 0, VERTICES.len() as i32);
        }
    }
}
