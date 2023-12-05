
use std::path::PathBuf;

pub enum MenuResult {
    Open(PathBuf),
    Save(PathBuf),
    ImportObj(PathBuf),
}

pub fn ui(ui: &mut egui::Ui, control_flow: &mut winit::event_loop::ControlFlow) -> Option<MenuResult> {
    let mut result = None;
    
    ui.menu_button("Menu", |ui| {
        if ui.button("Open").clicked() {
            log::debug!("Menu > Open");
            result = open().map(|path| MenuResult::Open(path));
        }

        if ui.button("Save").clicked() {
            log::debug!("Menu > Save");
            result = save().map(|path| MenuResult::Save(path));
        }

        ui.menu_button("Import", |ui| {
            if ui.button("Waveform (.obj)").clicked() {
                log::debug!("Menu > Import > Waveform");
                result = load().map(|path| MenuResult::ImportObj(path));
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

        if let Some(_) = result {
            ui.close_menu();
        }
    });

    result
}

fn open() -> Option<PathBuf> {
    native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
        .add_filter("Phobia part", &["ph"])
        .show_open_single_file()
        .unwrap_or_default()
}

fn save() -> Option<PathBuf> {
    native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
        .add_filter("Phobia part", &["ph"])
        .show_save_single_file()
        .unwrap_or_default()

        // set extension
        .map(|mut path| {
            path.set_extension("ph");
            path
        })
}

fn load() -> Option<PathBuf> {
    native_dialog::FileDialog::new()
        .set_location(&std::env::current_dir().unwrap())
        .add_filter("Wavefront", &["obj"])
        .show_open_single_file()
        .unwrap_or_default()
}