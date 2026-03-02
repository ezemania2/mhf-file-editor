use super::*;
use egui;
use crate::model::mhfdat::{SigilRecipe, SigilSkillProbabilities};
use crate::utils::sigil_skills_name::sigil_skill_name;

const PAGE_SIZE: usize = 20;

impl MhfdatApp {
    fn get_item_name_sigil(&self, id: u16) -> String {
        if id == 0 {
            "None".into()
        } else {
            self.item_names
                .get(id as usize)
                .cloned()
                .unwrap_or_else(|| format!("Item {}", id))
        }
    }

    pub fn show_sigils_tab(&mut self, ui: &mut egui::Ui) {
        if !self.view_mode.contains_key("sigils") {
            self.view_mode.insert("sigils".to_string(), ViewMode::List);
        }

        match self.view_mode.get("sigils").cloned().unwrap_or(ViewMode::List) {
            ViewMode::List => self.show_sigil_list(ui),
            ViewMode::Details => self.show_sigil_details(ui),
        }
    }

    fn show_sigil_list(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, &format!("Sigil Crafting Recipes ({} entries)", self.sigil_recipes.len()), |ui| {
            if ui.button("Export to JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_filename("sigil_recipes.json")
                    .show_save_single_file()
                {
                    #[derive(serde::Serialize)]
                    struct SigilExport {
                        idx: usize,
                        recipe: SigilRecipe,
                        probabilities: SigilSkillProbabilities,
                        blacklist: Vec<u16>,
                    }
                    let export: Vec<SigilExport> = self.sigil_recipes.iter().enumerate().map(|(i, r)| {
                        SigilExport {
                            idx: i,
                            recipe: r.clone(),
                            probabilities: self.sigil_probabilities.get(i).cloned().unwrap_or_default(),
                            blacklist: self.sigil_blacklists.get(i).cloned().unwrap_or_default(),
                        }
                    }).collect();
                    if let Ok(text) = serde_json::to_string_pretty(&export) {
                        let _ = std::fs::write(path, text);
                    }
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file()
                {
                    #[derive(serde::Deserialize)]
                    struct SigilExport {
                        recipe: SigilRecipe,
                        probabilities: SigilSkillProbabilities,
                        blacklist: Vec<u16>,
                    }
                    if let Ok(data) = std::fs::read_to_string(path) {
                        if let Ok(imported) = serde_json::from_str::<Vec<SigilExport>>(&data) {
                            self.sigil_recipes = imported.iter().map(|e| e.recipe.clone()).collect();
                            self.sigil_probabilities = imported.iter().map(|e| e.probabilities.clone()).collect();
                            self.sigil_blacklists = imported.iter().map(|e| e.blacklist.clone()).collect();
                            self.sigil_recipes_modified = true;
                        }
                    }
                }
            }
            if ui.button("Add New").clicked() {
                self.sigil_recipes.push(SigilRecipe::default());
                self.sigil_probabilities.push(SigilSkillProbabilities::default());
                self.sigil_blacklists.push(Vec::new());
                self.selected_sigil_recipe_index = Some(self.sigil_recipes.len() - 1);
                self.view_mode.insert("sigils".to_string(), ViewMode::Details);
                self.sigil_recipes_modified = true;
            }
        });

        let total = self.sigil_recipes.len();
        let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
        let page = (self.sigil_page as usize).min(total_pages.saturating_sub(1));
        if page != self.sigil_page as usize {
            self.sigil_page = page as u32;
        }
        MhfdatApp::pagination_controls(ui, &mut self.sigil_page, total_pages);

        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(total);

        let display_data: Vec<_> = (start..end).map(|i| {
            let recipe = &self.sigil_recipes[i];
            let mat_names: Vec<String> = recipe.key_materials.iter()
                .filter(|m| m.item != 0)
                .map(|m| self.get_item_name_sigil(m.item))
                .collect();
            let mats = if mat_names.is_empty() { "None".to_string() } else { mat_names.join(", ") };
            (i, recipe.cost, mats, recipe.extra_skills_low, recipe.extra_skills_high)
        }).collect();

        MhfdatApp::list_scroll(ui, "sigil_list_scroll", |ui| {
            egui::Grid::new("sigil_list_grid")
                .num_columns(4)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("ID");
                    ui.label("Cost");
                    ui.label("Materials");
                    ui.label("Skills Low/High");
                    ui.end_row();

                    for (i, cost, mats, skills_low, skills_high) in &display_data {
                        let selected = self.selected_sigil_recipe_index == Some(*i);
                        if ui.selectable_label(selected, format!("{}", i)).clicked() {
                            self.selected_sigil_recipe_index = Some(*i);
                            self.view_mode.insert("sigils".to_string(), ViewMode::Details);
                        }
                        ui.label(format!("{}", cost));
                        ui.label(mats);
                        ui.label(format!("{}/{}", skills_low, skills_high));
                        ui.end_row();
                    }
                });
        });
    }

    fn show_sigil_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("sigils".to_string(), ViewMode::List);
            return;
        }

        let idx = match self.selected_sigil_recipe_index {
            Some(i) if i < self.sigil_recipes.len() => i,
            _ => {
                self.view_mode.insert("sigils".to_string(), ViewMode::List);
                return;
            }
        };

        ui.heading(format!("Edit Sigil Recipe #{}", idx));
        ui.separator();

        let mut changed = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Recipe");
            egui::Grid::new("sigil_recipe_grid").num_columns(2).striped(true).show(ui, |ui| {
                let recipe = &mut self.sigil_recipes[idx];

                ui.label("Cost:");
                if ui.add(egui::DragValue::new(&mut recipe.cost)).changed() { changed = true; }
                ui.end_row();

                ui.label("Extra Skills Low:");
                if ui.add(egui::DragValue::new(&mut recipe.extra_skills_low)).changed() { changed = true; }
                ui.end_row();

                ui.label("Extra Skills High:");
                if ui.add(egui::DragValue::new(&mut recipe.extra_skills_high)).changed() { changed = true; }
                ui.end_row();

                ui.label("Unk0:");
                if ui.add(egui::DragValue::new(&mut recipe.unk0)).changed() { changed = true; }
                ui.end_row();

                ui.label("Unk1:");
                if ui.add(egui::DragValue::new(&mut recipe.unk1)).changed() { changed = true; }
                ui.end_row();
            });

            ui.add_space(8.0);
            ui.heading("Key Materials");
            egui::Grid::new("sigil_materials_grid").num_columns(5).striped(true).show(ui, |ui| {
                ui.strong("#");
                ui.strong("Item");
                ui.strong("Item ID");
                ui.strong("% Filled");
                ui.strong("Unk");
                ui.end_row();

                let recipe = &mut self.sigil_recipes[idx];
                for j in 0..5 {
                    ui.label(format!("{}", j + 1));
                    let item_name = self.item_names.get(recipe.key_materials[j].item as usize)
                        .cloned().unwrap_or_else(|| format!("Item {}", recipe.key_materials[j].item));
                    ui.label(&item_name);
                    if ui.add(egui::DragValue::new(&mut recipe.key_materials[j].item)).changed() { changed = true; }
                    if ui.add(egui::DragValue::new(&mut recipe.key_materials[j].percentage_filled)).changed() { changed = true; }
                    if ui.add(egui::DragValue::new(&mut recipe.key_materials[j].unk)).changed() { changed = true; }
                    ui.end_row();
                }
            });

            while self.sigil_probabilities.len() <= idx {
                self.sigil_probabilities.push(SigilSkillProbabilities::default());
            }

            ui.add_space(8.0);
            ui.heading("Skill Probabilities");
            egui::Grid::new("sigil_probs_grid").num_columns(5).striped(true).show(ui, |ui| {
                ui.strong("Skill");
                ui.strong("Skill ID");
                ui.strong("Chance %");
                ui.strong("Low Pts");
                ui.strong("High Pts");
                ui.end_row();

                let probs = &mut self.sigil_probabilities[idx];
                for j in 0..8 {
                    let p = &mut probs.probabilities[j];
                    let name = sigil_skill_name(p.skill);
                    ui.label(name);
                    if ui.add(egui::DragValue::new(&mut p.skill)).changed() { changed = true; }
                    if ui.add(egui::DragValue::new(&mut p.percentage_chance)).changed() { changed = true; }
                    if ui.add(egui::DragValue::new(&mut p.low_points)).changed() { changed = true; }
                    if ui.add(egui::DragValue::new(&mut p.high_points)).changed() { changed = true; }
                    ui.end_row();
                }
            });

            while self.sigil_blacklists.len() <= idx {
                self.sigil_blacklists.push(Vec::new());
            }

            ui.add_space(8.0);
            ui.heading("Skill Blacklist");
            ui.horizontal(|ui| {
                if ui.button("Add Skill to Blacklist").clicked() {
                    self.sigil_blacklists[idx].push(0);
                    changed = true;
                }
            });

            let mut remove_idx: Option<usize> = None;
            egui::Grid::new("sigil_blacklist_grid").num_columns(3).striped(true).show(ui, |ui| {
                ui.strong("#");
                ui.strong("Skill");
                ui.strong("");
                ui.end_row();

                let blacklist = &mut self.sigil_blacklists[idx];
                for j in 0..blacklist.len() {
                    ui.label(format!("{}", j));
                    let name = sigil_skill_name(blacklist[j]);
                    ui.horizontal(|ui| {
                        ui.label(name);
                        if ui.add(egui::DragValue::new(&mut blacklist[j])).changed() { changed = true; }
                    });
                    if ui.button("Remove").clicked() {
                        remove_idx = Some(j);
                        changed = true;
                    }
                    ui.end_row();
                }
            });

            if let Some(ri) = remove_idx {
                self.sigil_blacklists[idx].remove(ri);
            }

            ui.separator();
            if ui.button("Delete this entry").clicked() {
                self.sigil_recipes.remove(idx);
                if idx < self.sigil_probabilities.len() { self.sigil_probabilities.remove(idx); }
                if idx < self.sigil_blacklists.len() { self.sigil_blacklists.remove(idx); }
                self.sigil_recipes_modified = true;
                self.selected_sigil_recipe_index = None;
                self.view_mode.insert("sigils".to_string(), ViewMode::List);
                return;
            }
        });

        if changed {
            self.sigil_recipes_modified = true;
        }
    }
}
