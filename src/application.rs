
use support::camera::CameraState;

use glium::uniform;
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use crate::support::camera;
use crate::{
    shaders,
    support::{self, ApplicationContext}
};

pub struct Application {
    pub vertex_buffer: glium::vertex::VertexBufferAny,
    pub program: glium::Program,
    pub camera: CameraState,
}

impl ApplicationContext for Application {
    const WINDOW_TITLE:&'static str = "3d Modeler";

    fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_buffer = support::load_wavefront(&display, crate::models::CUBE);

        let program = glium::Program::from_source(
            display, shaders::VERTEX_SRC, shaders::FRAGMENT_SRC, None
        ).unwrap();

        Self {
            vertex_buffer,
            program,
            camera: CameraState::new(),
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
        self.camera.update(camera::UPDATE_DISTANCE);
    }
}
