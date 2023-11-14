
use glium::{program, uniform};
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use crate::support::{self, ApplicationContext};

pub struct Application {
    pub vertex_buffer: glium::vertex::VertexBufferAny,
    pub program: glium::Program,
    pub camera: support::camera::CameraState,
}

impl ApplicationContext for Application {
    const WINDOW_TITLE:&'static str = "3d Modeler";

    fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_buffer = support::load_wavefront(&display, include_bytes!("obj/cube.obj"));
        let vertex_shader_src = r#"
            #version 140

            uniform mat4 persp_matrix;
            uniform mat4 view_matrix;

            in vec3 position;
            in vec3 normal;
            out vec3 v_position;
            out vec3 v_normal;

            void main() {
                v_position = position;
                v_normal = normal;
                gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec3 v_normal;
            out vec4 f_color;

            const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

            void main() {
                float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                vec3 color = (0.3 + 0.7 * lum) * vec3(0.89, 0.51, 0.49);
                f_color = vec4(color, 1.0);
            }
        "#;
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src,
            None).unwrap();

        let camera = support::camera::CameraState::new();

        Self {
            vertex_buffer,
            program,
            camera,
        }
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: self.camera.get_perspective(),
            view_matrix: self.camera.get_view(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        frame.clear_color_and_depth((0.18, 0.25, 0.4, 1.0), 1.0);
        frame
            .draw(
                &self.vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program,
                &uniforms,
                &params,
            )
            .unwrap();
        frame.finish().unwrap();
    }

    fn handle_window_event(&mut self, event: &winit::event::WindowEvent, _window: &winit::window::Window) {
        self.camera.process_input(&event);
    }

    fn update(&mut self) {
        self.camera.update();
    }
}
