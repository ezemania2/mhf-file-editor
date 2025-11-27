use eframe::egui::{self, Ui};
use std::path::PathBuf;

use super::{RandomizerSeed};
use super::weapons::{randomize_melee_buffer_with_options, randomize_ranged_buffer_with_options, MeleeWeaponOptions as WeaponMeleeOptions, RangedWeaponOptions as WeaponRangedOptions};
use super::armor::{randomize_armor_buffer_with_options, ArmorOptions as ArmorOptionsInternal};
use super::upgrades::{randomize_upgrades_buffer_with_options, UpgradeOptions as UpgradeOptionsInternal};
use crate::core::packing::{compress_file, encrypt_file};

#[derive(Default)]
pub struct RandomizerOptions {
    pub randomize_melee: bool,
    pub randomize_ranged: bool,
    pub randomize_armor: bool,
    pub randomize_upgrades: bool,
}

#[derive(Default)]
pub struct MeleeWeaponOptions {
    pub randomize_elements: bool,
    pub randomize_status: bool,
    pub randomize_sharpness: bool,
    pub randomize_zenith_skills: bool,
}

#[derive(Default)]
pub struct RangedWeaponOptions {
    pub randomize_elements: bool,
    pub randomize_bullet_types: bool,
    pub randomize_weapon_attributes: bool,
    pub randomize_zenith_skills: bool,
}

#[derive(Default)]
pub struct ArmorOptions {
    pub randomize_skills: bool,
    pub randomize_skill_points: bool,
    pub randomize_zenith_skills: bool,
    pub randomize_resistances: bool,
    pub randomize_slots: bool,
}

#[derive(Default)]
pub struct UpgradeOptions {
    pub randomize_materials: bool,
    pub randomize_targets: bool,
}

pub struct RandomizerApp {
    pub files: Vec<PathBuf>,
    pub seed_input: String,
    pub options: RandomizerOptions,
    pub melee_options: MeleeWeaponOptions,
    pub ranged_options: RangedWeaponOptions,
    pub armor_options: ArmorOptions,
    pub upgrade_options: UpgradeOptions,
    pub status_message: String,
    pub should_return_to_selector: bool,
}

impl Default for RandomizerApp {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            seed_input: String::new(),
            options: RandomizerOptions::default(),
            melee_options: MeleeWeaponOptions::default(),
            ranged_options: RangedWeaponOptions::default(),
            armor_options: ArmorOptions::default(),
            upgrade_options: UpgradeOptions::default(),
            status_message: String::new(),
            should_return_to_selector: false,
        }
    }
}

impl RandomizerApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                // Unload files from memory
                self.files.clear();
                self.seed_input.clear();
                self.options = RandomizerOptions::default();
                self.melee_options = MeleeWeaponOptions::default();
                self.ranged_options = RangedWeaponOptions::default();
                self.armor_options = ArmorOptions::default();
                self.upgrade_options = UpgradeOptions::default();
                self.status_message.clear();
                
                self.should_return_to_selector = true;
            }
            ui.heading("Monster Hunter Frontier Randomizer");
        });
        ui.separator();

        // File selection
        ui.horizontal(|ui| {
            ui.label("Files to randomize:");
            if ui.button("Add Files").clicked() {
                if let Some(paths) = rfd::FileDialog::new()
                    .add_filter("MHF Data", &["bin"])
                    .pick_files()
                {
                    for path in paths {
                        if !self.files.contains(&path) {
                            self.files.push(path);
                        }
                    }
                }
            }
            ui.label("ℹ").on_hover_text(
                "Add Monster Hunter Frontier data files to randomize:\n\
                • mhfdat.bin - Main game data (weapons, armor, items)\n\
                This file is typically found in your dat folder."
            );
        });

        // Display selected files
        let mut to_remove = None;
        for (i, file) in self.files.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(file.file_name().unwrap_or_default().to_string_lossy());
                if ui.button("Remove").clicked() {
                    to_remove = Some(i);
                }
            });
        }
        if let Some(i) = to_remove {
            self.files.remove(i);
        }

        ui.separator();

        // Seed input
        ui.horizontal(|ui| {
            ui.label("Seed:");
            ui.text_edit_singleline(&mut self.seed_input);
            if ui.button("Randomize seed").on_hover_text("Generate random seed").clicked() {
                let random_seed = RandomizerSeed::random();
                self.seed_input = random_seed.value.to_string();
            }
        });

        ui.separator();

        // Main options
        ui.heading("Randomization Options");
        ui.checkbox(&mut self.options.randomize_melee, "Randomize Melee Weapons");
        ui.checkbox(&mut self.options.randomize_ranged, "Randomize Ranged Weapons");
        ui.checkbox(&mut self.options.randomize_armor, "Randomize Armor");
        ui.checkbox(&mut self.options.randomize_upgrades, "Randomize Upgrades");

        ui.separator();

        // Detailed options
        if self.options.randomize_melee {
            ui.collapsing("Melee Weapon Options", |ui| {
                ui.checkbox(&mut self.melee_options.randomize_elements, "Elements");
                ui.checkbox(&mut self.melee_options.randomize_status, "Status Effects");
                ui.checkbox(&mut self.melee_options.randomize_sharpness, "Sharpness");
                ui.checkbox(&mut self.melee_options.randomize_zenith_skills, "Zenith Skills");
            });
        }

        if self.options.randomize_ranged {
            ui.collapsing("Ranged Weapon Options", |ui| {
                ui.checkbox(&mut self.ranged_options.randomize_elements, "Elements");
                ui.checkbox(&mut self.ranged_options.randomize_bullet_types, "Bullet Types");
                ui.checkbox(&mut self.ranged_options.randomize_weapon_attributes, "Weapon Attributes");
                ui.checkbox(&mut self.ranged_options.randomize_zenith_skills, "Zenith Skills");
            });
        }

        if self.options.randomize_armor {
            ui.collapsing("Armor Options", |ui| {
                ui.checkbox(&mut self.armor_options.randomize_skills, "Skills");
                ui.checkbox(&mut self.armor_options.randomize_skill_points, "Skill Points");
                ui.checkbox(&mut self.armor_options.randomize_zenith_skills, "Zenith Skills");
                ui.checkbox(&mut self.armor_options.randomize_resistances, "Resistances");
                ui.checkbox(&mut self.armor_options.randomize_slots, "Decoration Slots");
            });
        }

        if self.options.randomize_upgrades {
            ui.collapsing("Upgrade Options", |ui| {
                ui.checkbox(&mut self.upgrade_options.randomize_materials, "Materials");
                ui.checkbox(&mut self.upgrade_options.randomize_targets, "Targets");
            });
        }

        ui.separator();

        // Action buttons
        ui.horizontal(|ui| {
            if ui.button("Randomize").clicked() {
                self.randomize_files();
            }
            
            if ui.button("Compress").clicked() {
                self.compress_files();
            }
            
            if ui.button("Encrypt").clicked() {
                self.encrypt_files();
            }
        });

        // Status message
        if !self.status_message.is_empty() {
            ui.separator();
            ui.label(&self.status_message);
        }
    }

    fn randomize_files(&mut self) {
        if self.files.is_empty() {
            self.status_message = "No files selected.".to_string();
            return;
        }

        let seed = RandomizerSeed::new(&self.seed_input);
        let mut rng = seed.create_rng();
        let mut total_processed = 0;

        for file_path in &self.files {
            match std::fs::read(file_path) {
                Ok(mut buffer) => {
                    let mut file_processed = false;

                    if self.options.randomize_melee {
                        let options = WeaponMeleeOptions {
                            randomize_elements: self.melee_options.randomize_elements,
                            randomize_status: self.melee_options.randomize_status,
                            randomize_sharpness: self.melee_options.randomize_sharpness,
                            randomize_zenith_skills: self.melee_options.randomize_zenith_skills,
                        };
                        if let Ok(_) = randomize_melee_buffer_with_options(&mut buffer, &mut rng, &options) {
                            file_processed = true;
                        }
                    }

                    if self.options.randomize_ranged {
                        let options = WeaponRangedOptions {
                            randomize_elements: self.ranged_options.randomize_elements,
                            randomize_bullet_types: self.ranged_options.randomize_bullet_types,
                            randomize_weapon_attributes: self.ranged_options.randomize_weapon_attributes,
                            randomize_zenith_skills: self.ranged_options.randomize_zenith_skills,
                        };
                        if let Ok(_) = randomize_ranged_buffer_with_options(&mut buffer, &mut rng, &options) {
                            file_processed = true;
                        }
                    }

                    if self.options.randomize_armor {
                        let options = ArmorOptionsInternal {
                            randomize_skills: self.armor_options.randomize_skills,
                            randomize_skill_points: self.armor_options.randomize_skill_points,
                            randomize_zenith_skills: self.armor_options.randomize_zenith_skills,
                            randomize_resistances: self.armor_options.randomize_resistances,
                            randomize_slots: self.armor_options.randomize_slots,
                        };
                        if let Ok(_) = randomize_armor_buffer_with_options(&mut buffer, &mut rng, &options) {
                            file_processed = true;
                        }
                    }

                    if self.options.randomize_upgrades {
                        let options = UpgradeOptionsInternal {
                            randomize_materials: self.upgrade_options.randomize_materials,
                            randomize_targets: self.upgrade_options.randomize_targets,
                        };
                        if let Ok(_) = randomize_upgrades_buffer_with_options(&mut buffer, &mut rng, &options) {
                            file_processed = true;
                        }
                    }

                    if file_processed {
                        if let Err(e) = std::fs::write(file_path, &buffer) {
                            self.status_message = format!("Error writing file {}: {}", file_path.display(), e);
                            return;
                        }
                        total_processed += 1;
                    }
                }
                Err(e) => {
                    self.status_message = format!("Error reading file {}: {}", file_path.display(), e);
                    return;
                }
            }
        }

        self.status_message = format!("Successfully randomized {} files with seed: {}", total_processed, seed.display_string);
    }

    fn compress_files(&mut self) {
        if self.files.is_empty() {
            self.status_message = "No files selected.".to_string();
            return;
        }

        let mut total_processed = 0;

        for file_path in &self.files {
            let temp_path = file_path.with_extension("tmp");
            if let Err(e) = std::fs::copy(file_path, &temp_path) {
                self.status_message = format!("Error creating temp file for compression: {}", e);
                return;
            }
            if let Err(e) = compress_file(&temp_path, file_path) {
                self.status_message = format!("Error compressing file {}: {}", file_path.display(), e);
                let _ = std::fs::remove_file(&temp_path);
                return;
            }
            let _ = std::fs::remove_file(&temp_path);
            total_processed += 1;
        }

        self.status_message = format!("Successfully compressed {} files with JPK Type 4.", total_processed);
    }

    fn encrypt_files(&mut self) {
        if self.files.is_empty() {
            self.status_message = "No files selected.".to_string();
            return;
        }

        let mut total_processed = 0;

        for file_path in &self.files {
            let temp_path = file_path.with_extension("tmp");
            if let Err(e) = std::fs::copy(file_path, &temp_path) {
                self.status_message = format!("Error creating temp file for encryption: {}", e);
                return;
            }
            if let Err(e) = encrypt_file(&temp_path, file_path) {
                self.status_message = format!("Error encrypting file {}: {}", file_path.display(), e);
                let _ = std::fs::remove_file(&temp_path);
                return;
            }
            let _ = std::fs::remove_file(&temp_path);
            total_processed += 1;
        }

        self.status_message = format!("Successfully encrypted {} files with ECD.", total_processed);
    }
}

impl eframe::App for RandomizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show(ui);
        });
    }
}
