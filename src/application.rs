
use support::camera::CameraState;

use glium::{
    uniform,
    {Display, Surface},
    vertex::VertexBufferAny,
    glutin::surface::WindowSurface,
};
use crate::support::camera;
use crate::{
    shaders,
    support::{self, ApplicationContext}
};

#[derive(Debug)]
pub struct Application {
    pub vertex_buffer: Option<VertexBufferAny>,
    pub program: glium::Program,
    pub camera: CameraState,

    model: Option<(Obj, bool)>
}

impl Application {
    fn open(&mut self) {
        let path = native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
            .add_filter("Wavefront", &["obj"])
            .show_open_single_file()
            .unwrap_or_default();
        
        if let Some(path) = path {
            self.model = Some((Obj::load(path).unwrap(), true))
        };
    }
}

impl ApplicationContext for Application {
    const WINDOW_TITLE:&'static str = crate::config::TITLE;

    fn new(display: &Display<WindowSurface>) -> Self {
        let program = glium::Program::from_source(
            display, shaders::VERTEX_SRC, shaders::FRAGMENT_SRC, None
        ).unwrap();

        Self {
            vertex_buffer: None,
            program,
            camera: CameraState::new(),
            model: None
        }
    }

    fn init(&mut self) {
        self.open();
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        if let None = self.vertex_buffer { () }

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
                self.vertex_buffer.as_ref().unwrap(),
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

    fn update_model(&mut self, display: &Display<WindowSurface>) {
        if let Some((model, true)) = &self.model {
            self.vertex_buffer = Some(support::load_wavefront(display, &model));
        }
    }
}
