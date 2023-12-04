
use egui::{Context, Layout, Align};
use egui_glium::EguiGlium;

use glium::{ uniform, Surface};
use winit::event_loop::ControlFlow;
use winit::event::WindowEvent;

use crate::prelude::*;
use crate::{
    model::Model,
    shaders,
    camera::{self, CameraState},
    state::ApplicationContext
};

#[derive(Debug)]
pub struct Application {
    pub program: glium::Program,
    pub camera: CameraState,

    model: Model,
    status: String
}

impl Application {
    fn open(&mut self) {
        let path = native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
            .add_filter("Phobia part", &["ph"])
            .show_open_single_file()
            .unwrap_or_default();
        
        if let Some(path) = path {
            self.status = format!("model {} loaded", &path.to_str().unwrap().to_string());
            let _ = self.model.load(path);
        };
    }

    fn save(&mut self) {
        let path = native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
        .add_filter("Phobia part", &["ph"])
            .show_save_single_file()
            .unwrap_or_default();
        
        if let Some(mut path) = path {
            path.set_extension("ph");

            // TODO: fix this. Obj saves faces with textures, not vertex normals
            match self.model.save(&path) {
                Ok(_) => self.status = format!("model {} saved", &path.to_str().unwrap()),
                Err(e) => self.status = format!("model save failed: {}", e)
            }
        };
    }

    fn load(&mut self) {
        let path = native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
            .add_filter("Wavefront", &["obj"])
            .show_open_single_file()
            .unwrap_or_default();
        
        if let Some(path) = path {
            self.model.load_obj(&path);
            self.status = format!("model {} loaded", &path.to_str().unwrap().to_string());
        };
    }
}

impl ApplicationContext for Application {
    const WINDOW_TITLE:&'static str = crate::config::TITLE;

    fn new(display: &Display) -> Self {
        let program = glium::Program::from_source(
            display, shaders::VERTEX_SRC, shaders::FRAGMENT_SRC, None
        ).unwrap();

        Self {
            program,
            camera: CameraState::new(),
            model: Model::new(),
            status: String::from("no model loaded"),
        }
    }

    fn draw_ui(&mut self, ctx: &Context, control_flow: &mut ControlFlow) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Open").clicked() {
                        self.open();
                        ui.close_menu();
                        
                        ctx.request_repaint();
                    }
    
                    if ui.button("Save").clicked() {
                        self.save();
                        ui.close_menu();
                        
                        ctx.request_repaint();
                    }
    
                    ui.menu_button("Import", |ui| {
                        if ui.button("Waveform (.obj)").clicked() {
                            self.load();
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
                });

                ui.horizontal(|ui| {
                    if ui.button("+ Sketch").clicked() {
                        eprintln!("sketcher not implemented");
                    }
                });
    
                #[cfg(debug_assertions)]
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("quick").clicked() {
                        self.model.load_obj(&std::path::PathBuf::from(crate::dev::QUICK_MODEL));
                    }
                });
            });
        });

        // TODO: model history panel
        // egui::SidePanel::left("toolbar").show(ctx, |ui| {
        // });

        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(&format!(
                    "🔄 <{:.2}, {:.2}, {:.2}> | ↔ <{:.2}, {:.2}, {:.2}>",
                    self.camera.rotation.0,
                    self.camera.rotation.1,
                    self.camera.rotation.2,
                    self.camera.position.0,
                    self.camera.position.1,
                    self.camera.position.2,
                ));

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(&self.status);
                });
            });
        });
    }

    fn draw_frame(&mut self, display: &Display, egui_glium_ctx: &mut EguiGlium) {
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

    fn handle_window_event(&mut self, event: &WindowEvent, _window: &winit::window::Window) {
        self.camera.process_input(&event);
    }

    fn update(&mut self) {
        self.camera.update(camera::UPDATE_DISTANCE);
    }
}
