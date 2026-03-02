use super::*;
use std::fs;

const PAGE_SIZE: usize = 15;

impl MhfdatApp {
    pub fn show_bullet_sets_tab(&mut self, ui: &mut egui::Ui) {
        if !self.view_mode.contains_key("bullet_sets") {
            self.view_mode.insert("bullet_sets".to_string(), ViewMode::List);
        }

        ui.horizontal(|ui| {
            ui.heading("Bullet Sets Editor");
            if ui.button("Export to JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_filename("bullet_sets.json")
                    .show_save_single_file() 
                {
                    if let Ok(json) = MhfdatApp::export_indexed_json(&self.bullet_sets) {
                        let _ = fs::write(path.to_str().unwrap_or("bullet_sets.json"), json);
                    }
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file() 
                {
                    if let Ok(data) = fs::read_to_string(path.to_str().unwrap_or("")) {
                        if let Ok(imported) = serde_json::from_str::<Vec<crate::model::mhfdat::BulletSet>>(&data) {
                            self.bullet_sets = imported;
                            self.bullet_sets_modified = true;
                        }
                    }
                }
            }
        });
        ui.separator();

        match self.view_mode.get("bullet_sets").unwrap_or(&ViewMode::List) {
            ViewMode::List => self.show_bullet_sets_list(ui),
            ViewMode::Details => self.show_bullet_set_details_view(ui),
        }
    }

    fn show_bullet_sets_list(&mut self, ui: &mut egui::Ui) {
        if self.bullet_sets.is_empty() {
            ui.label("No bullet sets data loaded.");
            return;
        }

        let total = self.bullet_sets.len();
        let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
        let page = (self.bullet_sets_page as usize).min(total_pages.saturating_sub(1));

        ui.label(format!("Total bullet sets: {}", total));
        MhfdatApp::pagination_controls(ui, &mut self.bullet_sets_page, total_pages);
        ui.separator();

        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(total);

        let display_data: Vec<_> = (start..end).map(|idx| {
            let s = &self.bullet_sets[idx];
            (
                idx,
                format!("{}/{}/{}", s.normal_lv1_capacity, s.normal_lv2_capacity, s.normal_lv3_capacity),
                format!("{}/{}/{}", s.pierce_lv1_capacity, s.pierce_lv2_capacity, s.pierce_lv3_capacity),
                format!("{}/{}/{}", s.spread_lv1_capacity, s.spread_lv2_capacity, s.spread_lv3_capacity),
                format!("{}/{}/{}", s.crag_lv1_capacity, s.crag_lv2_capacity, s.crag_lv3_capacity),
                format!("{}/{}/{}", s.cluster_lv1_capacity, s.cluster_lv2_capacity, s.cluster_lv3_capacity),
            )
        }).collect();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("bullet_sets_list_grid").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Normal Lv1-3");
                ui.label("Pierce Lv1-3");
                ui.label("Spread Lv1-3");
                ui.label("Crag Lv1-3");
                ui.label("Cluster Lv1-3");
                ui.end_row();

                for (idx, normal, pierce, spread, crag, cluster) in display_data {
                    let selected = self.selected_bullet_set_id == Some(idx);
                    if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                        self.selected_bullet_set_id = Some(idx);
                        self.view_mode.insert("bullet_sets".to_string(), ViewMode::Details);
                    }
                    ui.label(normal);
                    ui.label(pierce);
                    ui.label(spread);
                    ui.label(crag);
                    ui.label(cluster);
                    ui.end_row();
                }
            });
        });
    }

    fn show_bullet_set_details_view(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("bullet_sets".to_string(), ViewMode::List);
            return;
        }
        ui.separator();

        let Some(id) = self.selected_bullet_set_id else {
            ui.label("Select a bullet set ID to edit.");
            return;
        };

        if id >= self.bullet_sets.len() {
            ui.label("Selected ID is out of range.");
            return;
        }

        let set = &mut self.bullet_sets[id];
        ui.heading(format!("Bullet Set ID: {}", id));
        ui.separator();

        let mut modified = false;

        ui.horizontal(|ui| {
            // Left column
            ui.vertical(|ui| {
                egui::Grid::new("bullet_main").striped(true).num_columns(4).show(ui, |ui| {
                    ui.label("Type"); ui.label("Lv1"); ui.label("Lv2"); ui.label("Lv3"); ui.end_row();
                    
                    ui.label("Normal");
                    modified |= ui.add(egui::DragValue::new(&mut set.normal_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.normal_lv2_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.normal_lv3_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                    
                    ui.label("Pierce");
                    modified |= ui.add(egui::DragValue::new(&mut set.pierce_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.pierce_lv2_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.pierce_lv3_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                    
                    ui.label("Spread");
                    modified |= ui.add(egui::DragValue::new(&mut set.spread_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.spread_lv2_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.spread_lv3_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                    
                    ui.label("Crag");
                    modified |= ui.add(egui::DragValue::new(&mut set.crag_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.crag_lv2_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.crag_lv3_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                    
                    ui.label("Cluster");
                    modified |= ui.add(egui::DragValue::new(&mut set.cluster_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.cluster_lv2_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.cluster_lv3_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                });

                ui.add_space(10.0);
                
                egui::Grid::new("bullet_special").striped(true).num_columns(2).show(ui, |ui| {
                    ui.label("Special"); ui.label("Capacity"); ui.end_row();
                    ui.label("Tranquilizer"); modified |= ui.add(egui::DragValue::new(&mut set.tranquilizer_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                    ui.label("Paint"); modified |= ui.add(egui::DragValue::new(&mut set.paint_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                    ui.label("Demon"); modified |= ui.add(egui::DragValue::new(&mut set.demon_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                    ui.label("Armor"); modified |= ui.add(egui::DragValue::new(&mut set.armor_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                });
            });

            ui.separator();

            // Right column
            ui.vertical(|ui| {
                egui::Grid::new("bullet_elements").striped(true).num_columns(2).show(ui, |ui| {
                    ui.label("Element"); ui.label("Capacity"); ui.end_row();
                    ui.label("Fire"); modified |= ui.add(egui::DragValue::new(&mut set.fire_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                    ui.label("Water"); modified |= ui.add(egui::DragValue::new(&mut set.water_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                    ui.label("Thunder"); modified |= ui.add(egui::DragValue::new(&mut set.thunder_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                    ui.label("Ice"); modified |= ui.add(egui::DragValue::new(&mut set.ice_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                    ui.label("Dragon"); modified |= ui.add(egui::DragValue::new(&mut set.dragon_capacity).clamp_range(0..=255)).changed(); ui.end_row();
                });

                ui.add_space(10.0);
                
                egui::Grid::new("bullet_status").striped(true).num_columns(3).show(ui, |ui| {
                    ui.label("Status"); ui.label("Lv1"); ui.label("Lv2"); ui.end_row();
                    ui.label("Recovery");
                    modified |= ui.add(egui::DragValue::new(&mut set.recovery_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.recovery_lv2_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                    ui.label("Poison");
                    modified |= ui.add(egui::DragValue::new(&mut set.poison_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.poison_lv2_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                    ui.label("Paralysis");
                    modified |= ui.add(egui::DragValue::new(&mut set.paralysis_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.paralysis_lv2_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                    ui.label("Sleep");
                    modified |= ui.add(egui::DragValue::new(&mut set.sleep_lv1_capacity).clamp_range(0..=255)).changed();
                    modified |= ui.add(egui::DragValue::new(&mut set.sleep_lv2_capacity).clamp_range(0..=255)).changed();
                    ui.end_row();
                });
            });
        });

        if modified {
            self.bullet_sets_modified = true;
        }
    }
}
