
use egui::Context;
use egui_glium::EguiGlium;
use support::camera::CameraState;

use glium::{
    uniform,
    {Display, Surface},
    glutin::surface::WindowSurface,
};
use winit::event_loop::ControlFlow;
use crate::{
    model::Model,
    shaders,
    support::{self, camera, ApplicationContext}
};

#[derive(Debug)]
pub struct Application {
    pub program: glium::Program,
    pub camera: CameraState,

    model: Model
}

impl Application {
    fn open(&mut self) {
        let path = native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
            .add_filter("Wavefront", &["obj"])
            .show_open_single_file()
            .unwrap_or_default();
        
        if let Some(path) = path {
            self.model.load(path);
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
            program,
            camera: CameraState::new(),
            model: Model::new()
        }
    }

    fn init(&mut self) {
        ()
    }

    fn draw_menu(&mut self, ctx: &Context, control_flow: &mut ControlFlow) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.menu_button("Menu", |ui| {
                if ui.button("Open").clicked() {
                    eprintln!("impl open of native file");
                    ui.close_menu();
                }

                ui.menu_button("Import", |ui| {
                    if ui.button("Waveform (.obj)").clicked() {
                        self.open();
                        ui.close_menu();
                        
                        ctx.request_repaint();
                    }
                });
                ui.menu_button("Export", |ui| {
                    if ui.button("Stereolithography (.stl)").clicked() {
                        eprintln!("impl stl export menu button");
                        ui.close_menu();
                    }
                });

                if ui.button("Quit").clicked() {
                    control_flow.set_exit();
                }
            })
        });
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>, egui_glium_ctx: &mut EguiGlium) {
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
                self.model.vertex_buffer(display),
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program,
                &uniforms,
                &params,
            )
            .unwrap();

        // draw egui header
        egui_glium_ctx.paint(display, &mut frame);

        frame.finish().unwrap();
    }

    fn handle_window_event(&mut self, event: &winit::event::WindowEvent, _window: &winit::window::Window) {
        self.camera.process_input(&event);
    }

    fn update(&mut self) {
        self.camera.update(camera::UPDATE_DISTANCE);
    }
}
