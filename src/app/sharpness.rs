use super::*;

impl MhfdatApp {
    pub fn show_sharpness_tab(&mut self, ui: &mut egui::Ui) {
        // Initialize view mode if not present
        if !self.view_mode.contains_key("sharpness") {
            self.view_mode.insert("sharpness".to_string(), ViewMode::List);
        }

        ui.heading("Sharpness Editor");
        ui.separator();

        // Weapon type selector
        // Ensure selected_sharpness_weapon_type is within valid range (0-11)
        if self.selected_sharpness_weapon_type >= 12 {
            self.selected_sharpness_weapon_type = 0;
        }
        
        ui.horizontal(|ui| {
            ui.label("Weapon Type:");
            egui::ComboBox::from_label("")
                .selected_text(Self::get_sharpness_weapon_type_name(self.selected_sharpness_weapon_type))
                .show_ui(ui, |ui| {
                    for i in 0..12 {
                        ui.selectable_value(&mut self.selected_sharpness_weapon_type, i, Self::get_sharpness_weapon_type_name(i));
                    }
                });
        });

        ui.separator();

        // Show list or details view
        match self.view_mode.get("sharpness").unwrap_or(&ViewMode::List) {
            ViewMode::List => self.show_sharpness_list(ui),
            ViewMode::Details => self.show_sharpness_details_view(ui),
        }
    }

    fn show_sharpness_list(&mut self, ui: &mut egui::Ui) {
        let weapon_type_idx = self.selected_sharpness_weapon_type;
        
        // Get the current weapon type's sharpness data (immutable borrow)
        let sharpness_data = match weapon_type_idx {
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
        };
        
        if sharpness_data.is_empty() {
            ui.label("No sharpness data loaded for this weapon type.");
            return;
        }

        let total = sharpness_data.len();
        let page_size = 15;
        let total_pages = (total + page_size - 1) / page_size;
        let page = (self.sharpness_page as usize).min(total_pages.saturating_sub(1));
        
        ui.label(format!("Total sharpness entries: {}", total));
        MhfdatApp::pagination_controls(ui, &mut self.sharpness_page, total_pages);
        ui.separator();

        let start = page * page_size;
        let end = (start + page_size).min(total);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("sharpness_list_grid")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("ID");
                    ui.label("Preview");
                    ui.end_row();

                    for idx in start..end {
                        let item = &sharpness_data[idx];
                        let selected = self.selected_sharpness_id == Some(idx);
                        if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                            self.selected_sharpness_id = Some(idx);
                            self.view_mode.insert("sharpness".to_string(), ViewMode::Details);
                        }
                        // Mini preview bar
                        Self::render_mini_sharpness_bar(ui, item);
                        ui.end_row();
                    }
                });
        });
    }

    fn show_sharpness_details_view(&mut self, ui: &mut egui::Ui) {
        // Back button
        ui.horizontal(|ui| {
            if ui.button("← Back to List").clicked() {
                if let Some(mode) = self.view_mode.get_mut("sharpness") {
                    *mode = ViewMode::List;
                }
            }
        });
        ui.separator();

        let weapon_type_idx = self.selected_sharpness_weapon_type;
        let selected_id = self.selected_sharpness_id;

        // Get the current weapon type's sharpness data
        let sharpness_data = self.get_sharpness_data_mut(weapon_type_idx);
        
        if sharpness_data.is_empty() {
            ui.label("No sharpness data loaded for this weapon type.");
            return;
        }

        // Show sharpness details
        if let Some(id) = selected_id {
            if id < sharpness_data.len() {
                let mut was_modified = false;
                Self::show_sharpness_details(ui, &mut sharpness_data[id], id, weapon_type_idx, &mut was_modified);
                if was_modified {
                    self.sharpness_modified[weapon_type_idx] = true;
                }
            } else {
                ui.label("Selected ID is out of range.");
            }
        } else {
            ui.label("Select a sharpness ID to edit.");
        }
    }

    fn render_mini_sharpness_bar(ui: &mut egui::Ui, item: &crate::model::mhfdat::SharpnessItem) {
        let total = item.total() as f32;
        if total > 0.0 {
            let bar_width = 200.0;
            let bar_height = 16.0;
            
            let (rect, _response) = ui.allocate_exact_size(
                egui::vec2(bar_width, bar_height),
                egui::Sense::hover()
            );

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                let mut x_offset = rect.left();
                
                // Build color segments (same logic as main bar)
                let mut color_segments = Vec::new();
                
                if item.red > 0 {
                    color_segments.push((item.red as f32, egui::Color32::from_rgb(255, 0, 0)));
                }
                if item.red < 400 && item.orange > 0 {
                    color_segments.push((item.orange as f32, egui::Color32::from_rgb(255, 165, 0)));
                }
                if item.red < 400 && item.orange < 400 && item.yellow > 0 {
                    color_segments.push((item.yellow as f32, egui::Color32::from_rgb(255, 255, 0)));
                }
                if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green > 0 {
                    color_segments.push((item.green as f32, egui::Color32::from_rgb(0, 255, 0)));
                }
                if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue > 0 {
                    color_segments.push((item.blue as f32, egui::Color32::from_rgb(0, 0, 255)));
                }
                if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue < 400 && item.white > 0 {
                    color_segments.push((item.white as f32, egui::Color32::from_rgb(255, 255, 255)));
                }
                if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue < 400 && item.white < 400 && item.purple > 0 {
                    color_segments.push((item.purple as f32, egui::Color32::from_rgb(128, 0, 128)));
                }
                if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue < 400 && item.white < 400 && item.purple < 400 && item.sky_blue > 0 {
                    color_segments.push((item.sky_blue as f32, egui::Color32::from_rgb(135, 206, 235)));
                }

                let total_used = color_segments.iter().map(|(v, _)| *v).sum::<f32>().max(1.0);
                
                for (value, color) in &color_segments {
                    if *value > 0.0 {
                        let width = (*value / total_used) * bar_width;
                        let segment_rect = egui::Rect::from_min_size(
                            egui::pos2(x_offset, rect.top()),
                            egui::vec2(width, bar_height)
                        );
                        painter.rect_filled(segment_rect, 0.0, *color);
                        x_offset += width;
                    }
                }

                // Draw border
                painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
            }
        } else {
            ui.label("(Empty)");
        }
    }

    fn show_sharpness_details(ui: &mut egui::Ui, item: &mut crate::model::mhfdat::SharpnessItem, id: usize, _weapon_type: usize, was_modified: &mut bool) {
        const MAX_SHARPNESS: f32 = 400.0;
        
        ui.heading(format!("Sharpness ID: {}", id));
        ui.separator();

        // Visual representation
        ui.label("Sharpness Bar:");
        let total = item.total() as f32;
        if total > 0.0 {
            ui.horizontal(|ui| {
                let bar_width = 400.0;
                let bar_height = 30.0;
                
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(bar_width, bar_height),
                    egui::Sense::hover()
                );

                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    
                    let mut x_offset = rect.left();
                    
                    // Draw each sharpness color segment (only show colors up to the first one that reached 400)
                    // Build the color segments: stop when we reach a color that has 400 (100%)
                    let mut color_segments = Vec::new();
                    
                    if item.red > 0 {
                        color_segments.push((item.red as f32, egui::Color32::from_rgb(255, 0, 0)));
                    }
                    if item.red < 400 && item.orange > 0 {
                        color_segments.push((item.orange as f32, egui::Color32::from_rgb(255, 165, 0)));
                    }
                    if item.red < 400 && item.orange < 400 && item.yellow > 0 {
                        color_segments.push((item.yellow as f32, egui::Color32::from_rgb(255, 255, 0)));
                    }
                    if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green > 0 {
                        color_segments.push((item.green as f32, egui::Color32::from_rgb(0, 255, 0)));
                    }
                    if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue > 0 {
                        color_segments.push((item.blue as f32, egui::Color32::from_rgb(0, 0, 255)));
                    }
                    if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue < 400 && item.white > 0 {
                        color_segments.push((item.white as f32, egui::Color32::from_rgb(255, 255, 255)));
                    }
                    // Stop here if white reached 400 - don't show purple and sky blue in the bar
                    if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue < 400 && item.white < 400 && item.purple > 0 {
                        color_segments.push((item.purple as f32, egui::Color32::from_rgb(128, 0, 128)));
                    }
                    if item.red < 400 && item.orange < 400 && item.yellow < 400 && item.green < 400 && item.blue < 400 && item.white < 400 && item.purple < 400 && item.sky_blue > 0 {
                        color_segments.push((item.sky_blue as f32, egui::Color32::from_rgb(135, 206, 235)));
                    }

                    // Calculate total for percentage display
                    let total_used = color_segments.iter().map(|(v, _)| *v).sum::<f32>().max(1.0);
                    
                    for (value, color) in &color_segments {
                        if *value > 0.0 {
                            let width = (*value / total_used) * bar_width;
                            let segment_rect = egui::Rect::from_min_size(
                                egui::pos2(x_offset, rect.top()),
                                egui::vec2(width, bar_height)
                            );
                            painter.rect_filled(segment_rect, 0.0, *color);
                            x_offset += width;
                        }
                    }

                    // Draw border
                    painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
                }

                response
            });
        } else {
            ui.label("(Empty sharpness bar)");
        }

        ui.separator();

        // Editable values
        egui::Grid::new("sharpness_grid").striped(true).show(ui, |ui| {
            ui.label("Color");
            ui.label("Value");
            ui.label("Percentage");
            ui.end_row();

            // Copy values to avoid packed field issues
            let mut red = item.red;
            let mut orange = item.orange;
            let mut yellow = item.yellow;
            let mut green = item.green;
            let mut blue = item.blue;
            let mut white = item.white;
            let mut purple = item.purple;
            let mut sky_blue = item.sky_blue;
            
            // Always show all colors in the editable grid
            Self::render_sharpness_field(ui, "Red", &mut red, MAX_SHARPNESS);
            Self::render_sharpness_field(ui, "Orange", &mut orange, MAX_SHARPNESS);
            Self::render_sharpness_field(ui, "Yellow", &mut yellow, MAX_SHARPNESS);
            Self::render_sharpness_field(ui, "Green", &mut green, MAX_SHARPNESS);
            Self::render_sharpness_field(ui, "Blue", &mut blue, MAX_SHARPNESS);
            Self::render_sharpness_field(ui, "White", &mut white, MAX_SHARPNESS);
            Self::render_sharpness_field(ui, "Purple", &mut purple, MAX_SHARPNESS);
            Self::render_sharpness_field(ui, "Sky Blue", &mut sky_blue, MAX_SHARPNESS);
            
            // Write back
            item.red = red;
            item.orange = orange;
            item.yellow = yellow;
            item.green = green;
            item.blue = blue;
            item.white = white;
            item.purple = purple;
            item.sky_blue = sky_blue;
            
            // Mark that sharpness was modified
            *was_modified = true;
        });

        ui.separator();
        let total = item.total();
        ui.label(format!("Total: {} / 400 ({:.1}%)", total, (total as f32 / MAX_SHARPNESS) * 100.0));
    }

    fn render_sharpness_field(ui: &mut egui::Ui, label: &str, value: &mut u16, total: f32) {
        ui.label(label);
        ui.add(egui::DragValue::new(value).speed(1.0).clamp_range(0..=400));
        if total > 0.0 {
            ui.label(format!("{:.1}%", (*value as f32 / total) * 100.0));
        } else {
            ui.label("0%");
        }
        ui.end_row();
    }

    fn get_sharpness_weapon_type_name(index: usize) -> &'static str {
        match index {
            0 => "Great Sword",
            1 => "Hammer",
            2 => "Lance",
            3 => "Sword and Shield",
            4 => "Dual Blades",
            5 => "Long Sword",
            6 => "Hunting Horn",
            7 => "Gunlance",
            8 => "Bow",
            9 => "Tonfa",
            10 => "Switch Axe",
            11 => "Magnet Spike",
            _ => "Unknown",
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
            _ => &mut self.sharpness.great_sword, // fallback
        }
    }
}

