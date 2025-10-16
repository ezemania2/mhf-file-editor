#![windows_subsystem = "windows"]

mod model;
pub mod utils;
mod app;
pub mod core;
mod randomizer;

use eframe::{NativeOptions, egui};
use ico::IconDir;
use std::sync::atomic::{AtomicBool, Ordering};
use rfd;

// Include resources directly in the binary
const NOTO_FONT: &[u8] = include_bytes!("../assets/NotoSansCJKjp-Regular.otf");
const APP_ICON: &[u8] = include_bytes!("../assets/app.ico");

// Variable globale pour le contrôle de l'arrêt
static RUNNING: AtomicBool = AtomicBool::new(true);

enum AppState {
    FileTypeSelector,
    Mhfjmp(app::MhfjmpApp),
    Mhfdat(app::MhfdatApp),
    Randomizer(randomizer::RandomizerApp),
}

pub struct RootApp {
    state: AppState,
    recent_files: Vec<String>,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {
            state: AppState::FileTypeSelector,
            recent_files: Vec::new(),
        }
    }
}

impl eframe::App for RootApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Vérifier si l'application doit s'arrêter
        if !RUNNING.load(Ordering::Relaxed) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Initialize fonts
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto".to_owned(),
            egui::FontData::from_owned(NOTO_FONT.to_vec()),
        );
        for family in [
            egui::FontFamily::Proportional,
            egui::FontFamily::Monospace,
            egui::FontFamily::Name("noto".into()),
        ] {
            fonts.families.entry(family).or_default().insert(0, "noto".to_owned());
        }
        ctx.set_fonts(fonts);

        

        match &mut self.state {
            AppState::FileTypeSelector => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Select file type to open:");

                    // Tools
                    ui.add_space(10.0);
                    if ui.button("Open Randomizer").clicked() {
                        self.state = AppState::Randomizer(randomizer::RandomizerApp::default());
                    }
                    ui.separator();

                    // File type selection buttons
                    if ui.button("Open mhfjmp").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("MHFJMP Files", &["bin"])
                            .set_title("Select MHFJMP file")
                            .pick_file() 
                        {
                            // Vérifier que le nom du fichier contient "mhfjmp" (insensible à la casse)
                            if let Some(file_name) = path.file_name() {
                                let file_name_str = file_name.to_string_lossy().to_lowercase();
                                if file_name_str.contains("mhfjmp") {
                                    if let Ok(data) = std::fs::read(&path) {
                                        let mut app = app::MhfjmpApp::default();
                                        let state_ptr = self as *mut RootApp;
                                        app.on_back = Some(Box::new(move || unsafe {
                                            (*state_ptr).state = AppState::FileTypeSelector;
                                        }));
                                        let opened_file = path.display().to_string();
                                        app.load_file(path, data);
                                        self.state = AppState::Mhfjmp(app);
                                        
                                        // Ajouter aux fichiers récents
                                        if !self.recent_files.contains(&opened_file) {
                                            self.recent_files.insert(0, opened_file.clone());
                                            if self.recent_files.len() > 5 {
                                                self.recent_files.pop();
                                            }
                                        }
                                    }
                                } else {
                                    // Afficher un message d'erreur si le fichier ne contient pas "mhfjmp" dans le nom
                                    eprintln!("Error: Selected file must contain 'mhfjmp' in its name");
                                }
                            }
                        }
                    }
                    
                    if ui.button("Open mhfdat").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("MHFDAT Files", &["bin"])
                            .set_title("Select MHFDAT file")
                            .pick_file() 
                        {
                            // Vérifier que le nom du fichier contient "mhfdat" (insensible à la casse)
                            if let Some(file_name) = path.file_name() {
                                let file_name_str = file_name.to_string_lossy().to_lowercase();
                                if file_name_str.contains("mhfdat") {
                                    if let Ok(data) = std::fs::read(&path) {
                                        let mut app = app::MhfdatApp::default();
                                        let opened_file = path.display().to_string();
                                        app.load_file(path, data);
                                        self.state = AppState::Mhfdat(app);
                                        
                                        // Ajouter aux fichiers récents
                                        if !self.recent_files.contains(&opened_file) {
                                            self.recent_files.insert(0, opened_file.clone());
                                            if self.recent_files.len() > 5 {
                                                self.recent_files.pop();
                                            }
                                        }
                                    }
                                } else {
                                    // Afficher un message d'erreur si le fichier ne contient pas "mhfdat" dans le nom
                                    eprintln!("Error: Selected file must contain 'mhfdat' in its name");
                                }
                            }
                        }
                    }

                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.heading("About");
                        ui.label("MHF File Editor v1.0");
                        ui.label("Developed by Ezemania");
                        ui.add_space(5.0);
                        ui.label("This software allows you to open, view and edit binary files from Monster Hunter Frontier (mhfjmp.bin, mhfdat.bin).\n\nOpen a file, explore, filter, edit and save easily. All changes are directly applied to the selected file.\n\nHow to use:\n- Click on 'MHJMP' or 'MHFDAT' depending on your file type.\n- Open your .bin file.\n- Edit weapons or their properties.\n- Save your changes.\nDisclaimer, you have to use decrypted files before using it.\n\nThis software will be open-source and provided as-is, without warranty.");
                    });


                });
            }
            AppState::Mhfjmp(app) => app.update(ctx, frame),
            AppState::Mhfdat(app) => {
                app.update(ctx, frame);
                // Check if user wants to go back to file selector
                if app.should_return_to_selector {
                    self.state = AppState::FileTypeSelector;
                }
            }
            AppState::Randomizer(app) => app.update(ctx, frame),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    // Configurer le gestionnaire de signal
    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");

    let icon = load_icon();
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_icon(icon);

    let options = NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "MHF File Editor",
        options,
        Box::new(|_cc| Box::new(RootApp::default())),
    )
}

fn load_icon() -> egui::IconData {
    let icon_dir = IconDir::read(std::io::Cursor::new(APP_ICON))
        .expect("Failed to read icon data");
    let image = icon_dir.entries().first()
        .expect("No icon entries found")
        .decode()
        .expect("Failed to decode icon");
    let width = image.width();
    let height = image.height();
    let rgba = image.rgba_data().to_vec();
    egui::IconData {
        rgba,
        width,
        height,
    }
}

