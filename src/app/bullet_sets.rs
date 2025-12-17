use super::*;

impl MhfdatApp {
    pub fn show_bullet_sets_tab(&mut self, ui: &mut egui::Ui) {
        // Initialize view mode if not present
        if !self.view_mode.contains_key("bullet_sets") {
            self.view_mode.insert("bullet_sets".to_string(), ViewMode::List);
        }

        ui.heading("Bullet Sets Editor");
        ui.separator();

        // Show list or details view
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

        ui.label(format!("Total bullet sets: {}", self.bullet_sets.len()));
        ui.separator();

        // Bullet sets list
        egui::CollapsingHeader::new(format!("Bullet Sets List ({} entries)", self.bullet_sets.len()))
            .default_open(true)
            .show(ui, |ui| {
                MhfdatApp::list_scroll(ui, "bullet_sets_list_scroll", |ui| {
                    egui::Grid::new("bullet_sets_list_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("ID");
                            ui.label("Normal Lv1-3");
                            ui.label("Pierce Lv1-3");
                            ui.label("Spread Lv1-3");
                            ui.label("Crag Lv1-3");
                            ui.label("Cluster Lv1-3");
                            ui.end_row();

                            for (idx, set) in self.bullet_sets.iter().enumerate() {
                                let selected = self.selected_bullet_set_id == Some(idx);
                                if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                                    self.selected_bullet_set_id = Some(idx);
                                    if !self.view_mode.contains_key("bullet_sets") {
                                        self.view_mode.insert("bullet_sets".to_string(), ViewMode::List);
                                    }
                                    *self.view_mode.get_mut("bullet_sets").unwrap() = ViewMode::Details;
                                }
                                ui.label(format!("{}/{}/{}", set.normal_lv1_capacity, set.normal_lv2_capacity, set.normal_lv3_capacity));
                                ui.label(format!("{}/{}/{}", set.pierce_lv1_capacity, set.pierce_lv2_capacity, set.pierce_lv3_capacity));
                                ui.label(format!("{}/{}/{}", set.spread_lv1_capacity, set.spread_lv2_capacity, set.spread_lv3_capacity));
                                ui.label(format!("{}/{}/{}", set.crag_lv1_capacity, set.crag_lv2_capacity, set.crag_lv3_capacity));
                                ui.label(format!("{}/{}/{}", set.cluster_lv1_capacity, set.cluster_lv2_capacity, set.cluster_lv3_capacity));
                                ui.end_row();
                            }
                        });
                });
            });
    }

    fn show_bullet_set_details_view(&mut self, ui: &mut egui::Ui) {
        // Back button
        ui.horizontal(|ui| {
            if ui.button("← Back to List").clicked() {
                if let Some(mode) = self.view_mode.get_mut("bullet_sets") {
                    *mode = ViewMode::List;
                }
            }
        });
        ui.separator();

        let selected_id = self.selected_bullet_set_id;

        if self.bullet_sets.is_empty() {
            ui.label("No bullet sets data loaded.");
            return;
        }

        // Show bullet set details
        if let Some(id) = selected_id {
            if id < self.bullet_sets.len() {
                let mut was_modified = false;
                Self::show_bullet_set_details(ui, &mut self.bullet_sets[id], id, &mut was_modified);
                if was_modified {
                    self.bullet_sets_modified = true;
                }
            } else {
                ui.label("Selected ID is out of range.");
            }
        } else {
            ui.label("Select a bullet set ID to edit.");
        }
    }

    fn show_bullet_set_details(ui: &mut egui::Ui, set: &mut crate::model::mhfdat::BulletSet, id: usize, was_modified: &mut bool) {
        ui.heading(format!("Bullet Set ID: {}", id));
        ui.separator();

        // Copy values to avoid packed field issues
        let mut normal_lv1 = set.normal_lv1_capacity;
        let mut normal_lv2 = set.normal_lv2_capacity;
        let mut normal_lv3 = set.normal_lv3_capacity;
        let mut pierce_lv1 = set.pierce_lv1_capacity;
        let mut pierce_lv2 = set.pierce_lv2_capacity;
        let mut pierce_lv3 = set.pierce_lv3_capacity;
        let mut spread_lv1 = set.spread_lv1_capacity;
        let mut spread_lv2 = set.spread_lv2_capacity;
        let mut spread_lv3 = set.spread_lv3_capacity;
        let mut crag_lv1 = set.crag_lv1_capacity;
        let mut crag_lv2 = set.crag_lv2_capacity;
        let mut crag_lv3 = set.crag_lv3_capacity;
        let mut cluster_lv1 = set.cluster_lv1_capacity;
        let mut cluster_lv2 = set.cluster_lv2_capacity;
        let mut cluster_lv3 = set.cluster_lv3_capacity;
        let mut fire = set.fire_capacity;
        let mut water = set.water_capacity;
        let mut thunder = set.thunder_capacity;
        let mut ice = set.ice_capacity;
        let mut dragon = set.dragon_capacity;
        let mut recovery_lv1 = set.recovery_lv1_capacity;
        let mut recovery_lv2 = set.recovery_lv2_capacity;
        let mut poison_lv1 = set.poison_lv1_capacity;
        let mut poison_lv2 = set.poison_lv2_capacity;
        let mut paralysis_lv1 = set.paralysis_lv1_capacity;
        let mut paralysis_lv2 = set.paralysis_lv2_capacity;
        let mut sleep_lv1 = set.sleep_lv1_capacity;
        let mut sleep_lv2 = set.sleep_lv2_capacity;
        let mut tranquilizer = set.tranquilizer_capacity;
        let mut paint = set.paint_capacity;
        let mut demon = set.demon_capacity;
        let mut armor = set.armor_capacity;

        // Editable values in 2 columns
        ui.horizontal(|ui| {
            // Left column
            ui.vertical(|ui| {
                // Bullet types section
                egui::Grid::new("bullet_set_grid_left").striped(true).num_columns(4).show(ui, |ui| {
                    ui.label("Type");
                    ui.label("Lv1");
                    ui.label("Lv2");
                    ui.label("Lv3");
                    ui.end_row();

                    // Normal
                    ui.label("Normal");
                    if ui.add(egui::DragValue::new(&mut normal_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut normal_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut normal_lv3).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();

                    // Pierce
                    ui.label("Pierce");
                    if ui.add(egui::DragValue::new(&mut pierce_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut pierce_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut pierce_lv3).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();

                    // Spread
                    ui.label("Spread");
                    if ui.add(egui::DragValue::new(&mut spread_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut spread_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut spread_lv3).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();

                    // Crag
                    ui.label("Crag");
                    if ui.add(egui::DragValue::new(&mut crag_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut crag_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut crag_lv3).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();

                    // Cluster
                    ui.label("Cluster");
                    if ui.add(egui::DragValue::new(&mut cluster_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut cluster_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut cluster_lv3).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                });

                ui.add_space(10.0);

                // Special section
                egui::Grid::new("bullet_set_special").striped(true).num_columns(2).show(ui, |ui| {
                    ui.label("Special");
                    ui.label("Capacity");
                    ui.end_row();

                    ui.label("Tranquilizer");
                    if ui.add(egui::DragValue::new(&mut tranquilizer).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Paint");
                    if ui.add(egui::DragValue::new(&mut paint).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Demon");
                    if ui.add(egui::DragValue::new(&mut demon).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Armor");
                    if ui.add(egui::DragValue::new(&mut armor).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                });
            });

            ui.separator();

            // Right column - organized in separate grids for better alignment
            ui.vertical(|ui| {
                // Elements section
                egui::Grid::new("bullet_set_elements").striped(true).num_columns(2).show(ui, |ui| {
                    ui.label("Element");
                    ui.label("Capacity");
                    ui.end_row();

                    ui.label("Fire");
                    if ui.add(egui::DragValue::new(&mut fire).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Water");
                    if ui.add(egui::DragValue::new(&mut water).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Thunder");
                    if ui.add(egui::DragValue::new(&mut thunder).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Ice");
                    if ui.add(egui::DragValue::new(&mut ice).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Dragon");
                    if ui.add(egui::DragValue::new(&mut dragon).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                });

                ui.add_space(10.0);

                // Status section
                egui::Grid::new("bullet_set_status").striped(true).num_columns(3).show(ui, |ui| {
                    ui.label("Status");
                    ui.label("Lv1");
                    ui.label("Lv2");
                    ui.end_row();

                    ui.label("Recovery");
                    if ui.add(egui::DragValue::new(&mut recovery_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut recovery_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Poison");
                    if ui.add(egui::DragValue::new(&mut poison_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut poison_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Paralysis");
                    if ui.add(egui::DragValue::new(&mut paralysis_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut paralysis_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                    ui.label("Sleep");
                    if ui.add(egui::DragValue::new(&mut sleep_lv1).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    if ui.add(egui::DragValue::new(&mut sleep_lv2).speed(1.0).clamp_range(0..=255)).changed() { *was_modified = true; }
                    ui.end_row();
                });
            });
        });

        // Write back
        set.normal_lv1_capacity = normal_lv1;
        set.normal_lv2_capacity = normal_lv2;
        set.normal_lv3_capacity = normal_lv3;
        set.pierce_lv1_capacity = pierce_lv1;
        set.pierce_lv2_capacity = pierce_lv2;
        set.pierce_lv3_capacity = pierce_lv3;
        set.spread_lv1_capacity = spread_lv1;
        set.spread_lv2_capacity = spread_lv2;
        set.spread_lv3_capacity = spread_lv3;
        set.crag_lv1_capacity = crag_lv1;
        set.crag_lv2_capacity = crag_lv2;
        set.crag_lv3_capacity = crag_lv3;
        set.cluster_lv1_capacity = cluster_lv1;
        set.cluster_lv2_capacity = cluster_lv2;
        set.cluster_lv3_capacity = cluster_lv3;
        set.fire_capacity = fire;
        set.water_capacity = water;
        set.thunder_capacity = thunder;
        set.ice_capacity = ice;
        set.dragon_capacity = dragon;
        set.recovery_lv1_capacity = recovery_lv1;
        set.recovery_lv2_capacity = recovery_lv2;
        set.poison_lv1_capacity = poison_lv1;
        set.poison_lv2_capacity = poison_lv2;
        set.paralysis_lv1_capacity = paralysis_lv1;
        set.paralysis_lv2_capacity = paralysis_lv2;
        set.sleep_lv1_capacity = sleep_lv1;
        set.sleep_lv2_capacity = sleep_lv2;
        set.tranquilizer_capacity = tranquilizer;
        set.paint_capacity = paint;
        set.demon_capacity = demon;
        set.armor_capacity = armor;
    }
}

