use super::*;

const PAGE_SIZE: usize = 15;
const MAX_SHARPNESS: u16 = 400;

// Sharpness colors with their RGB values
const SHARPNESS_COLORS: [(egui::Color32, &str); 8] = [
    (egui::Color32::from_rgb(255, 0, 0), "Red"),
    (egui::Color32::from_rgb(255, 165, 0), "Orange"),
    (egui::Color32::from_rgb(255, 255, 0), "Yellow"),
    (egui::Color32::from_rgb(0, 255, 0), "Green"),
    (egui::Color32::from_rgb(0, 0, 255), "Blue"),
    (egui::Color32::from_rgb(255, 255, 255), "White"),
    (egui::Color32::from_rgb(128, 0, 128), "Purple"),
    (egui::Color32::from_rgb(135, 206, 235), "Sky Blue"),
];

const WEAPON_TYPES: [&str; 12] = [
    "Great Sword", "Hammer", "Lance", "Sword and Shield",
    "Dual Blades", "Long Sword", "Hunting Horn", "Gunlance",
    "Bow", "Tonfa", "Switch Axe", "Magnet Spike",
];

impl MhfdatApp {
    pub fn show_sharpness_tab(&mut self, ui: &mut egui::Ui) {
        if !self.view_mode.contains_key("sharpness") {
            self.view_mode.insert("sharpness".to_string(), ViewMode::List);
        }

        ui.horizontal(|ui| {
            ui.heading("Sharpness Editor");
            if ui.button("Export to JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_filename("sharpness.json")
                    .show_save_single_file() 
                {
                    if let Ok(json) = serde_json::to_string_pretty(&self.sharpness) {
                        let _ = std::fs::write(path.to_str().unwrap_or("sharpness.json"), json);
                    }
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file() 
                {
                    if let Ok(data) = std::fs::read_to_string(path.to_str().unwrap_or("")) {
                        if let Ok(imported) = serde_json::from_str::<crate::model::mhfdat::SharpnessCollection>(&data) {
                            self.sharpness = imported;
                            self.sharpness_modified = [true; 12];
                        }
                    }
                }
            }
        });
        ui.separator();

        if self.selected_sharpness_weapon_type >= 12 {
            self.selected_sharpness_weapon_type = 0;
        }

        ui.horizontal(|ui| {
            ui.label("Weapon Type:");
            egui::ComboBox::from_label("")
                .selected_text(WEAPON_TYPES[self.selected_sharpness_weapon_type])
                .show_ui(ui, |ui| {
                    for (i, name) in WEAPON_TYPES.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_sharpness_weapon_type, i, *name);
                    }
                });
        });

        ui.separator();

        match self.view_mode.get("sharpness").unwrap_or(&ViewMode::List) {
            ViewMode::List => self.show_sharpness_list(ui),
            ViewMode::Details => self.show_sharpness_details_view(ui),
        }
    }

    fn get_sharpness_data(&self, weapon_type: usize) -> &Vec<crate::model::mhfdat::SharpnessItem> {
        match weapon_type {
            0 => &self.sharpness.great_sword,
            1 => &self.sharpness.hammer,
            2 => &self.sharpness.lance,
            3 => &self.sharpness.sword_and_shield,
            4 => &self.sharpness.dual_blades,
            5 => &self.sharpness.long_sword,
            6 => &self.sharpness.hunting_horn,
            7 => &self.sharpness.gunlance,
            8 => &self.sharpness.bow,
            9 => &self.sharpness.tonfa,
            10 => &self.sharpness.switch_axe,
            11 => &self.sharpness.magnet_spike,
            _ => &self.sharpness.great_sword,
        }
    }

    fn get_sharpness_data_mut(&mut self, weapon_type: usize) -> &mut Vec<crate::model::mhfdat::SharpnessItem> {
        match weapon_type {
            0 => &mut self.sharpness.great_sword,
            1 => &mut self.sharpness.hammer,
            2 => &mut self.sharpness.lance,
            3 => &mut self.sharpness.sword_and_shield,
            4 => &mut self.sharpness.dual_blades,
            5 => &mut self.sharpness.long_sword,
            6 => &mut self.sharpness.hunting_horn,
            7 => &mut self.sharpness.gunlance,
            8 => &mut self.sharpness.bow,
            9 => &mut self.sharpness.tonfa,
            10 => &mut self.sharpness.switch_axe,
            11 => &mut self.sharpness.magnet_spike,
            _ => &mut self.sharpness.great_sword,
        }
    }

    fn show_sharpness_list(&mut self, ui: &mut egui::Ui) {
        let total = self.get_sharpness_data(self.selected_sharpness_weapon_type).len();
        
        if total == 0 {
            ui.label("No sharpness data loaded for this weapon type.");
            return;
        }

        let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
        let page = (self.sharpness_page as usize).min(total_pages.saturating_sub(1));

        ui.label(format!("Total sharpness entries: {}", total));
        MhfdatApp::pagination_controls(ui, &mut self.sharpness_page, total_pages);
        ui.separator();

        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(total);

        // Pre-collect data for display
        let display_data: Vec<_> = {
            let data = self.get_sharpness_data(self.selected_sharpness_weapon_type);
            (start..end).map(|idx| {
                let item = &data[idx];
                (idx, Self::build_color_segments(item))
            }).collect()
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("sharpness_list_grid").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Preview");
                ui.end_row();

                for (idx, segments) in display_data {
                    let selected = self.selected_sharpness_id == Some(idx);
                    if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                        self.selected_sharpness_id = Some(idx);
                        self.view_mode.insert("sharpness".to_string(), ViewMode::Details);
                    }
                    Self::render_sharpness_bar(ui, &segments, 200.0, 16.0);
                    ui.end_row();
                }
            });
        });
    }

    fn show_sharpness_details_view(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("sharpness".to_string(), ViewMode::List);
            return;
        }
        ui.separator();

        let weapon_type = self.selected_sharpness_weapon_type;
        let Some(id) = self.selected_sharpness_id else {
            ui.label("Select a sharpness ID to edit.");
            return;
        };

        let data_len = self.get_sharpness_data(weapon_type).len();
        if id >= data_len {
            ui.label("Selected ID is out of range.");
            return;
        }

        // Read current values
        let (segments, mut values, total) = {
            let item = &self.get_sharpness_data(weapon_type)[id];
            let segments = Self::build_color_segments(item);
            let values = [
                item.red, item.orange, item.yellow, item.green,
                item.blue, item.white, item.purple, item.sky_blue,
            ];
            let total = item.total();
            (segments, values, total)
        };

        ui.heading(format!("Sharpness ID: {}", id));
        ui.separator();

        // Visual bar
        ui.label("Sharpness Bar:");
        ui.horizontal(|ui| {
            Self::render_sharpness_bar(ui, &segments, 400.0, 30.0);
        });
        ui.separator();

        // Editable values
        egui::Grid::new("sharpness_grid").striped(true).show(ui, |ui| {
            ui.label("Color");
            ui.label("Value");
            ui.label("Percentage");
            ui.end_row();

            for (i, (color, name)) in SHARPNESS_COLORS.iter().enumerate() {
                ui.colored_label(*color, *name);
                ui.add(egui::DragValue::new(&mut values[i]).speed(1.0).clamp_range(0..=400));
                ui.label(format!("{:.1}%", (values[i] as f32 / MAX_SHARPNESS as f32) * 100.0));
                ui.end_row();
            }
        });

        // Write back
        {
            let item = &mut self.get_sharpness_data_mut(weapon_type)[id];
            item.red = values[0];
            item.orange = values[1];
            item.yellow = values[2];
            item.green = values[3];
            item.blue = values[4];
            item.white = values[5];
            item.purple = values[6];
            item.sky_blue = values[7];
        }

        self.sharpness_modified[weapon_type] = true;

        ui.separator();
        ui.label(format!("Total: {} / 400 ({:.1}%)", total, (total as f32 / MAX_SHARPNESS as f32) * 100.0));
    }

    fn build_color_segments(item: &crate::model::mhfdat::SharpnessItem) -> Vec<(f32, egui::Color32)> {
        let values = [
            item.red, item.orange, item.yellow, item.green,
            item.blue, item.white, item.purple, item.sky_blue,
        ];
        
        let mut segments = Vec::new();
        for (i, &val) in values.iter().enumerate() {
            if val > 0 {
                // Check if any previous color reached 400
                let prev_maxed = values[..i].iter().any(|&v| v >= MAX_SHARPNESS);
                if !prev_maxed {
                    segments.push((val as f32, SHARPNESS_COLORS[i].0));
                }
            }
        }
        segments
    }

    fn render_sharpness_bar(ui: &mut egui::Ui, segments: &[(f32, egui::Color32)], width: f32, height: f32) {
        let total: f32 = segments.iter().map(|(v, _)| *v).sum();
        if total <= 0.0 {
            ui.label("(Empty)");
            return;
        }

        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let mut x = rect.left();

            for (value, color) in segments {
                let segment_width = (*value / total) * width;
                let segment_rect = egui::Rect::from_min_size(
                    egui::pos2(x, rect.top()),
                    egui::vec2(segment_width, height),
                );
                painter.rect_filled(segment_rect, 0.0, *color);
                x += segment_width;
            }

            painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
        }
    }
}
