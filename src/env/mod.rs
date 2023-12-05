
mod modeling;
mod sketcher;

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub use modeling::Modeler;
pub use sketcher::Sketcher;

use crate::camera::CameraState;
type Camera = Arc<Mutex<CameraState>>;

pub enum ApplicationEnvironmentSwitch {
    EnterSketcher,
    ExitSketcher(Option<crate::model::ModelEntity>),
}

pub enum ApplicationEnvironmentType {
    Modeling(Modeler),
    Sketching(Sketcher),
}

impl Deref for ApplicationEnvironmentType {
    type Target = dyn ApplicationEnvironmentOps;

    fn deref(&self) -> &Self::Target {
        match self {
            ApplicationEnvironmentType::Modeling(modeler) => modeler,
            ApplicationEnvironmentType::Sketching(sketcher) => sketcher,
        }
    }
}

impl DerefMut for ApplicationEnvironmentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ApplicationEnvironmentType::Modeling(modeler) => modeler,
            ApplicationEnvironmentType::Sketching(sketcher) => sketcher,
        }
    }
}

pub struct ApplicationEnvironment {
    pub camera: Camera,

    env: ApplicationEnvironmentType
}

impl ApplicationEnvironment {
    pub fn new() -> Self {
        let camera = Arc::new(Mutex::new(CameraState::new()));
        let env_camera = camera.clone();

        Self {
            camera,   
            env: ApplicationEnvironmentType::Modeling(Modeler::new(env_camera))
        }
    }

    pub fn update(&mut self) {
        match self.camera.lock() {
            Ok(mut camera) => camera.update(crate::camera::UPDATE_DISTANCE),
            Err(e) => log::error!("Failed to lock camera to handle update because `{}`", e)
        }
    }

    pub fn process_input(&mut self, event: &winit::event::WindowEvent) {
        self.env.handle_window_event(&event);
    }
}

impl Deref for ApplicationEnvironment {
    type Target = ApplicationEnvironmentType;

    fn deref(&self) -> &Self::Target {
        &self.env    
    }
}

impl DerefMut for ApplicationEnvironment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.env
    }
}

pub trait ApplicationEnvironmentOps
    where Self: std::fmt::Debug
{
    fn draw_toolbar(&mut self, ui: &mut egui::Ui) -> Option<ApplicationEnvironmentSwitch>;
    fn handle_window_event(&mut self, event: &winit::event::WindowEvent);
}
