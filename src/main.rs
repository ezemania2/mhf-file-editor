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
    /// Last "could not open this file" message, drawn on the selector screen (stderr is invisible in the
    /// windowed build).
    open_error: Option<String>,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {
            state: AppState::FileTypeSelector,
            recent_files: Vec::new(),
            open_error: None,
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

        // Fonts are installed once per context, not every frame
        crate::utils::theme::install_fonts(ctx);

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
                            match open_mhfjmp(path.clone()) {
                                Ok(app) => {
                                    self.open_error = None;
                                    remember_recent(&mut self.recent_files, &path);
                                    self.state = AppState::Mhfjmp(app);
                                }
                                Err(e) => self.open_error = Some(e),
                            }
                        }
                    }

                    if ui.button("Open mhfdat").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("MHFDAT Files", &["bin"])
                            .set_title("Select MHFDAT file")
                            .pick_file()
                        {
                            match open_mhfdat(path.clone()) {
                                Ok(app) => {
                                    self.open_error = None;
                                    remember_recent(&mut self.recent_files, &path);
                                    self.state = AppState::Mhfdat(app);
                                }
                                Err(e) => self.open_error = Some(e),
                            }
                        }
                    }

                    if let Some(err) = &self.open_error {
                        ui.add_space(6.0);
                        ui.colored_label(egui::Color32::from_rgb(220, 70, 70), err);
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
            AppState::Mhfjmp(app) => {
                app.update(ctx, frame);
                // Check if user wants to go back to file selector
                if app.should_return_to_selector {
                    self.state = AppState::FileTypeSelector;
                }
            }
            AppState::Mhfdat(app) => {
                app.update(ctx, frame);
                // Check if user wants to go back to file selector
                if app.should_return_to_selector {
                    self.state = AppState::FileTypeSelector;
                }
            }
            AppState::Randomizer(app) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    app.show(ui);
                });
                // Check if user wants to go back to file selector
                if app.should_return_to_selector {
                    self.state = AppState::FileTypeSelector;
                }
            }
        }
    }
}

fn remember_recent(recent: &mut Vec<String>, path: &std::path::Path) {
    let opened = path.display().to_string();
    if !recent.contains(&opened) {
        recent.insert(0, opened);
        recent.truncate(5);
    }
}

/// Reads the file (stripping ECD/JPK wrappers only) and checks the name; every failure is a message
/// for the screen, never a panic.
fn read_clear(path: &std::path::Path, needle: &str) -> Result<Vec<u8>, String> {
    let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
    if !name.contains(needle) {
        return Err(format!("The selected file must contain '{needle}' in its name ({})", path.display()));
    }
    crate::core::packing::unpack_file(path).map_err(|e| format!("Cannot open {}: {e}", path.display()))
}

fn open_mhfjmp(path: std::path::PathBuf) -> Result<app::MhfjmpApp, String> {
    let data = read_clear(&path, "mhfjmp")?;
    let sha = crate::core::packing::sha256_hex(&data);
    let size = data.len();
    let mut app = app::MhfjmpApp::default();
    app.load_file(path.clone(), data);
    match app.error_message.take() {
        Some(e) if app.current_file.is_none() => {
            Err(format!("{e} ({}: {size} bytes, SHA-256 {sha})", path.display()))
        }
        other => {
            app.error_message = other;
            Ok(app)
        }
    }
}

fn open_mhfdat(path: std::path::PathBuf) -> Result<app::MhfdatApp, String> {
    let data = read_clear(&path, "mhfdat")?;
    app::MhfdatApp::open(path, data)
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

