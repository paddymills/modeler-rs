
use glium::uniform;
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

        let vertex_shader_src   = include_str!("shaders/vertex.glsl");
        let fragment_shader_src = include_str!("shaders/fragment.glsl");
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
            rotx_matrix: self.camera.get_x_rotation(),
            roty_matrix: self.camera.get_y_rotation(),
            rotz_matrix: self.camera.get_z_rotation(),
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
