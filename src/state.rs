
use egui_glium::EguiGlium;
use glium::{uniform, Surface};

use winit::{
    event_loop::ControlFlow,
    window::Window,
};

use crate::prelude::*;
use crate::{
    env,
    camera::CameraState,
    application::ApplicationState,
    model::Model
};


// cannot #[derive(Debug)] because EguiGlium does not implement Debug
pub struct State {
    program: glium::Program,
    ui: EguiGlium,
    camera: CameraState,
    
    env: Box<dyn env::ApplicationEnvironment>,
    
    model: Model,
    status: String,
}

impl State {
    // TODO: move open, save and load to Application
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

impl ApplicationState for State {
    fn new(display: &Display, window: &Window, event_loop: &super::application::EventLoop) -> Self {
        
        // TODO: I think this will unwrap on a shader build failure,
        //  so we should impl a test to ensure unwrap safety
        let program = glium::Program::from_source(
            display,
            crate::shaders::VERTEX_SRC,
            crate::shaders::FRAGMENT_SRC,
            None    // geometry shader
        ).unwrap();
        
        Self {
            program,
            ui: EguiGlium::new(&display, &window, &event_loop),
            camera: CameraState::new(),
            env: Box::new(env::Modeling::new()),
            model: Model::new(),
            status: String::from("no model loaded"),
        }
    }

    fn update(&mut self) {
        self.camera.update(crate::camera::UPDATE_DISTANCE);
    }

    fn handle_window_event(&mut self, event: &winit::event::WindowEvent, _window: &winit::window::Window) {
        if !self.ui.on_event(&event).consumed {
            self.camera.process_input(&event);
        }
        
    }

    fn draw_ui(&mut self, control_flow: &mut ControlFlow, window: &Window) {
        self.ui.run(&window, |ctx| {
            egui::TopBottomPanel::top("menu").show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    // TODO: move to dedicated menu struct/fn
                    ui.menu_button("Menu", |ui| {
                        if ui.button("Open").clicked() {
                            log::debug!("Menu > Open");
                            // self.open();
                            ui.close_menu();
                            
                            ctx.request_repaint();
                        }
        
                        if ui.button("Save").clicked() {
                            log::debug!("Menu > Save");
                            // self.save();
                            ui.close_menu();
                            
                            ctx.request_repaint();
                        }
        
                        ui.menu_button("Import", |ui| {
                            if ui.button("Waveform (.obj)").clicked() {
                                log::debug!("Menu > Import > Waveform");
                                // self.load();
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
                        self.env.draw_toolbar(ui);
                    });
        
                    #[cfg(debug_assertions)]
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("quick").clicked() {
                            self.model.load_obj(&std::path::PathBuf::from(crate::dev::QUICK_MODEL));
                        }
                    });
                });
            });
    
            // model history panel
            egui::SidePanel::left("toolbar").show(&ctx, |ui| {
                for geo in self.model.entities() {
                    ui.label(geo);
                }
            });
    
            egui::TopBottomPanel::bottom("statusbar").show(&ctx, |ui| {
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
    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(&self.status);
                    });
                });
            });
        });
    }

    fn draw_frame(&mut self, display: &Display) {
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
        self.ui.paint(display, &mut frame);

        frame.finish().unwrap();
    }
}