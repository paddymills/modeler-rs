
use std::ops::{Deref, DerefMut};

use egui_glium::EguiGlium;
use glium::{uniform, Surface};

use winit::{
    event_loop::ControlFlow,
    window::Window,
};

use crate::env::{ApplicationEnvironmentSwitch, ApplicationEnvironmentType};
use crate::prelude::*;
use crate::ui::menu::MenuResult;
use crate::{
    env,
    application::ApplicationState,
    model::Model
};


// cannot #[derive(Debug)] because EguiGlium does not implement Debug
pub struct State {
    program: glium::Program,
    ui: EguiGlium,
    
    env: env::ApplicationEnvironment,
    
    model: Model,
    status: String,
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
            env: env::ApplicationEnvironment::new(),
            model: Model::new(),
            status: String::from("no model loaded"),
        }
    }

    fn update(&mut self) {
        self.env.update();
    }

    fn handle_window_event(&mut self, event: &winit::event::WindowEvent, _window: &winit::window::Window) {
        if !self.ui.on_event(&event).consumed {
            self.env.process_input(&event);
        }
        
    }

    fn draw_ui(&mut self, control_flow: &mut ControlFlow, window: &Window) {
        self.ui.run(&window, |ctx| {
            egui::TopBottomPanel::top("menu").show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    // TODO: fix Obj save (saves faces with textures, not vertex normals)
                    if let Some(res) = crate::ui::menu::ui(ui, control_flow) {
                        // handle result
                        match res {
                            MenuResult::Open(path) => {
                                if let Err(e) = self.model.load(path) {
                                    log::error!("Failed to open part <{}>", e)
                                }
                            },
                            MenuResult::Save(path) => {
                                if let Err(e) = self.model.save(&path) {
                                    log::error!("Failed to save part <{}>", e)
                                }
                            },
                            MenuResult::ImportObj(path) => {
                                if let Err(e) = self.model.load_obj(&path) {
                                    log::error!("Failed to load Obj file part <{}>", e)
                                }
                            },
                        }
                        ctx.request_repaint();
                    }
    
                    ui.horizontal(|ui| {
                        if let Some(switch) = self.env.draw_toolbar(ui) {
                            *self.env.deref_mut() = match switch {
                                ApplicationEnvironmentSwitch::EnterSketcher => ApplicationEnvironmentType::Sketching(self.env.deref().into()),
                                ApplicationEnvironmentSwitch::ExitSketcher(sketch) => {
                                    if let Some(sketch) = sketch {
                                        self.model.push(sketch);
                                    }
                                    
                                    ApplicationEnvironmentType::Modeling(self.env.deref().into())
                                },
                            };
                        }
                    });
        
                    #[cfg(debug_assertions)]
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("quick").clicked() {
                            let _ = self.model.load_obj(&std::path::PathBuf::from(crate::dev::QUICK_MODEL));
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
                    let rotpos = match self.env.camera.lock() {
                        Ok(camera) => (
                            camera.rotation.0,
                            camera.rotation.1,
                            camera.rotation.2,
                            camera.position.0,
                            camera.position.1,
                            camera.position.2,
                        ),
                        Err(_) => ( 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, ),
                    };

                    ui.label(&format!(
                        "🔄 <{:.2}, {:.2}, {:.2}> | ↔ <{:.2}, {:.2}, {:.2}>",
                        rotpos.0, rotpos.1, rotpos.2, rotpos.3, rotpos.4, rotpos.5,
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
        let camera = self.env.camera.lock().unwrap();
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix:  camera.get_view(),
            rotx_matrix:  camera.get_x_rotation(),
            roty_matrix:  camera.get_y_rotation(),
            rotz_matrix:  camera.get_z_rotation(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },

            line_width: Some(5f32),

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