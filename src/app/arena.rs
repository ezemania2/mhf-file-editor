use super::*;
use crate::model::mhfdat::{ItemBlock, PointerBlock};

#[derive(PartialEq, Clone, Copy, Default)]
pub enum ArenaTab {
    #[default]
    Items,
    Equipment,
    Ammo,
    Stages,
    Tutorial,
}

/// Index picker for a list of pointer blocks: previous / next buttons around a drag value. Returns the index.
fn pick_block<T>(ui: &mut egui::Ui, label: &str, blocks: &[PointerBlock<T>], selected: &mut usize) -> usize {
    let last = blocks.len().saturating_sub(1);
    *selected = (*selected).min(last);
    ui.horizontal(|ui| {
        ui.label(label);
        if ui.button("<").clicked() && *selected > 0 {
            *selected -= 1;
        }
        ui.add(egui::DragValue::new(selected).clamp_range(0..=last));
        if ui.button(">").clicked() && *selected < last {
            *selected += 1;
        }
        ui.label(format!("of {}", blocks.len()));
    });
    *selected
}

/// Why a block cannot be edited as a list of its own, if it cannot.
fn block_note<T>(ui: &mut egui::Ui, blocks: &[PointerBlock<T>], sel: usize) -> bool {
    let b = &blocks[sel];
    if b.offset == 0 {
        ui.label("Null pointer: nothing to edit here.");
        return false;
    }
    ui.label(format!("List at file offset 0x{:X}", b.offset));
    if let Some(first) = b.alias_of {
        ui.label(format!("Same list as #{first}: edit it there."));
        return false;
    }
    true
}

fn show_item_blocks(ui: &mut egui::Ui, id: &str, blocks: &mut Vec<PointerBlock<ItemBlock>>, selected: &mut usize, names: &[String], changed: &mut bool) {
    if blocks.is_empty() {
        ui.label("No list loaded.");
        return;
    }
    let sel = pick_block(ui, "List", blocks, selected);
    if !block_note(ui, blocks, sel) {
        return;
    }
    ui.label("A list ends on a quantity of -1 (0xFFFF), which is written after the last item.");
    ui.horizontal(|ui| {
        if ui.button("Add item").clicked() {
            let row = blocks[sel].rows.last().cloned().unwrap_or(ItemBlock { quantity: 1, id: 0 });
            blocks[sel].rows.push(row);
            *changed = true;
        }
        if MhfdatApp::delete_last_button(ui, blocks[sel].rows.len(), None) {
            blocks[sel].rows.pop();
            *changed = true;
        }
    });
    egui::Grid::new((id, sel)).striped(true).show(ui, |ui| {
        for h in ["Idx", "Quantity", "Item ID", ""] {
            ui.label(h);
        }
        ui.end_row();
        for (i, it) in blocks[sel].rows.iter_mut().enumerate() {
            ui.label(i.to_string());
            *changed |= ui.add(egui::DragValue::new(&mut it.quantity).clamp_range(0..=0xFFFE)).changed();
            *changed |= ui.add(egui::DragValue::new(&mut it.id)).changed();
            ui.label(MhfdatApp::item_label(names, it.id));
            ui.end_row();
        }
    });
}

impl MhfdatApp {
    /// Arena tables: item lists (header 0x144), equipment (0x148), ammo lists (0x158), stage ids (0x154 / 0x14C) and the
    /// tutorial / sleep-learning equipment sets (0x15C). The item and ammo lists can grow (the list is moved to the end of the
    /// file on save); the other tables keep their size.
    pub fn show_arena_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "arena");
        ui.horizontal(|ui| {
            for (tab, label) in [
                (ArenaTab::Items, "Item lists"),
                (ArenaTab::Equipment, "Equipment"),
                (ArenaTab::Ammo, "Ammo lists"),
                (ArenaTab::Stages, "Stage ids"),
                (ArenaTab::Tutorial, "Tutorial sets"),
            ] {
                if ui.selectable_label(self.arena_tab == tab, label).clicked() {
                    self.arena_tab = tab;
                    self.arena_selected = 0;
                    self.arena_chunk = 0;
                    self.back_category_list("arena");
                }
            }
        });
        ui.separator();
        match self.arena_tab {
            ArenaTab::Items => self.show_arena_blocks(ui, true),
            ArenaTab::Ammo => self.show_arena_blocks(ui, false),
            ArenaTab::Equipment => self.show_arena_equipment(ui),
            ArenaTab::Stages => self.show_arena_stages(ui),
            ArenaTab::Tutorial => self.show_tutorial_sets(ui),
        }
    }

    fn show_arena_blocks(&mut self, ui: &mut egui::Ui, items: bool) {
        let key = "arena";
        let title = if items { "Item lists" } else { "Ammo lists" };
        if self.category_details(key) {
            if ui.button("← Back to List").clicked() {
                self.back_category_list(key);
                return;
            }
            let blocks = if items { &self.arena.items } else { &self.arena.ammo };
            let sel = self.arena_selected.min(blocks.len().saturating_sub(1));
            ui.heading(format!("Edit {title} #{sel}"));
            ui.separator();
            let mut changed = false;
            if items {
                show_item_blocks(ui, "arena_items", &mut self.arena.items, &mut self.arena_selected, &self.item_names, &mut changed);
            } else {
                show_item_blocks(ui, "arena_ammo", &mut self.arena.ammo, &mut self.arena_selected, &self.item_names, &mut changed);
            }
            if changed { self.arena_modified = true; }
            return;
        }
        let total = if items { self.arena.items.len() } else { self.arena.ammo.len() };
        MhfdatApp::section_header(ui, &format!("{title} (found: {total})"), |_| {});
        ui.label("Open a list to add items to it or remove its last one. The number of lists is fixed.");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No list loaded!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.arena_chunk, total);
        let rows: Vec<(usize, u32, usize)> = (start..end).map(|i| {
            let b = if items { &self.arena.items[i] } else { &self.arena.ammo[i] };
            (i, b.offset, b.rows.len())
        }).collect();
        MhfdatApp::entity_list(ui, if items { "arena_items_scroll" } else { "arena_ammo_scroll" }, "List", start, end, total, |ui| {
            egui::Grid::new((title, "list")).striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Offset");
                ui.label("Rows");
                ui.end_row();
                for (i, offset, rows) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.arena_selected = *i;
                        self.open_category_details(key);
                    }
                    ui.label(format!("0x{offset:X}"));
                    ui.label(rows.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.arena_chunk, total_pages);
    }

    fn show_arena_equipment(&mut self, ui: &mut egui::Ui) {
        if self.category_details("arena") {
            self.show_arena_equipment_details(ui);
            return;
        }
        let total = self.arena.equipment.len();
        MhfdatApp::section_header(ui, &format!("Equipment (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No arena equipment found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.arena_chunk, total);
        let rows: Vec<(usize, u32, usize)> = (start..end).map(|i| {
            let b = &self.arena.equipment[i];
            (i, b.offset, b.rows.len())
        }).collect();
        MhfdatApp::entity_list(ui, "arena_equipment_scroll", "Set", start, end, total, |ui| {
            egui::Grid::new("arena_equipment_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Offset");
                ui.label("Rows");
                ui.end_row();
                for (i, offset, n) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.arena_selected = *i;
                        self.open_category_details("arena");
                    }
                    ui.label(format!("0x{offset:X}"));
                    ui.label(n.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.arena_chunk, total_pages);
    }

    fn show_arena_equipment_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("arena");
            return;
        }
        if self.arena.equipment.is_empty() {
            ui.label("No arena equipment loaded.");
            return;
        }
        let sel = self.arena_selected.min(self.arena.equipment.len() - 1);
        ui.heading(format!("Edit Equipment set #{sel}"));
        ui.separator();
        if !block_note(ui, &self.arena.equipment, sel) {
            return;
        }
        let mut changed = false;
        egui::Grid::new(("arena_equipment", sel)).striped(true).show(ui, |ui| {
            for h in ["ID", "Weapon class", "Weapon ID", "Head", "Chest", "Arms", "Waist", "Legs", "Set idx", "Multi"] {
                ui.label(h);
            }
            ui.end_row();
            for (i, d) in self.arena.equipment[sel].rows.iter_mut().enumerate() {
                ui.label(i.to_string());
                changed |= ui.add(egui::DragValue::new(&mut d.weapon_class)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.weapon_id)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.head)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.chest)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.arms)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.waist)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.legs)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.set_idx)).changed();
                changed |= ui.add(egui::DragValue::new(&mut d.multi)).changed();
                ui.end_row();
            }
        });
        if changed { self.arena_modified = true; }
    }

    fn show_arena_stages(&mut self, ui: &mut egui::Ui) {
        if self.category_details("arena") {
            if ui.button("← Back to List").clicked() {
                self.back_category_list("arena");
                return;
            }
            let ids = if self.arena_chunk == 0 { &mut self.arena.stage_ids_0 } else { &mut self.arena.stage_ids_1 };
            let sel = self.arena_selected.min(ids.len().saturating_sub(1));
            ui.heading(format!("Edit Stage id #{sel}"));
            ui.separator();
            if ids.is_empty() {
                ui.label("No stage ids loaded.");
                return;
            }
            let mut changed = false;
            ui.horizontal(|ui| {
                ui.label("Stage ID:");
                changed |= ui.add(egui::DragValue::new(&mut ids[sel])).changed();
            });
            if changed { self.arena_modified = true; }
            return;
        }
        ui.horizontal(|ui| {
            if ui.selectable_label(self.arena_chunk == 0, "Chunk 0").clicked() { self.arena_chunk = 0; }
            if ui.selectable_label(self.arena_chunk != 0, "Chunk 1").clicked() { self.arena_chunk = 1; }
        });
        let chunk = if self.arena_chunk == 0 { 0 } else { 1 };
        let total = if chunk == 0 { self.arena.stage_ids_0.len() } else { self.arena.stage_ids_1.len() };
        MhfdatApp::section_header(ui, &format!("Stage ids (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No stage ids found!");
            return;
        }
        let ids: Vec<u16> = if chunk == 0 { self.arena.stage_ids_0.clone() } else { self.arena.stage_ids_1.clone() };
        MhfdatApp::entity_list(ui, "arena_stages_scroll", "Stage", 0, total, total, |ui| {
            egui::Grid::new("arena_stages_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Stage ID");
                ui.end_row();
                for i in 0..total {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.arena_selected = i;
                        self.open_category_details("arena");
                    }
                    ui.label(ids[i].to_string());
                    ui.end_row();
                }
            });
        });
    }

    fn show_tutorial_sets(&mut self, ui: &mut egui::Ui) {
        if self.category_details("arena") {
            self.show_tutorial_details(ui);
            return;
        }
        let total = self.arena.tutorial.len();
        MhfdatApp::section_header(ui, &format!("Tutorial sets (found: {total})"), |_| {});
        ui.label("Indexed by quest id, or by the weapon picked in sleep learning. Equip type 6 = melee, 7 = ranged.");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No tutorial set found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.arena_chunk, total);
        let rows: Vec<(usize, String)> = (start..end).map(|i| {
            let block = &self.arena.tutorial[i];
            let note = if block.offset == 0 {
                "null".to_string()
            } else if let Some(first) = block.alias_of {
                format!("same as #{first}")
            } else if let Some(t) = block.rows.first() {
                format!("type {} weapon {}", t.weapon_equip_type, t.weapon_id)
            } else {
                String::new()
            };
            (i, note)
        }).collect();
        MhfdatApp::entity_list(ui, "tutorial_sets_scroll", "Set", start, end, total, |ui| {
            egui::Grid::new("tutorial_sets_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Summary");
                ui.end_row();
                for (i, note) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.arena_selected = *i;
                        self.open_category_details("arena");
                    }
                    ui.label(note);
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.arena_chunk, total_pages);
    }

    fn show_tutorial_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("arena");
            return;
        }
        let Some(block) = self.arena.tutorial.get(self.arena_selected) else {
            ui.label("No tutorial set selected");
            return;
        };
        let sel = self.arena_selected;
        ui.heading(format!("Edit Tutorial set #{sel}"));
        ui.separator();
        if block.offset == 0 {
            ui.label("Null pointer: nothing to edit here.");
            return;
        }
        if let Some(first) = block.alias_of {
            ui.label(format!("Same set as #{first}: edit it there."));
            return;
        }
        let mut changed = false;
        let Some(t) = self.arena.tutorial[sel].rows.first_mut() else {
            ui.label("Empty set.");
            return;
        };
        ui.horizontal(|ui| { ui.label("Equip type:"); changed |= ui.add(egui::DragValue::new(&mut t.weapon_equip_type)).changed(); });
        ui.horizontal(|ui| { ui.label("Weapon ID:"); changed |= ui.add(egui::DragValue::new(&mut t.weapon_id)).changed(); });
        ui.horizontal(|ui| { ui.label("Weapon level:"); changed |= ui.add(egui::DragValue::new(&mut t.weapon_level)).changed(); });
        ui.horizontal(|ui| { ui.label("Head:"); changed |= ui.add(egui::DragValue::new(&mut t.head)).changed(); });
        ui.horizontal(|ui| { ui.label("Chest:"); changed |= ui.add(egui::DragValue::new(&mut t.chest)).changed(); });
        ui.horizontal(|ui| { ui.label("Arms:"); changed |= ui.add(egui::DragValue::new(&mut t.arms)).changed(); });
        ui.horizontal(|ui| { ui.label("Waist:"); changed |= ui.add(egui::DragValue::new(&mut t.waist)).changed(); });
        ui.horizontal(|ui| { ui.label("Legs:"); changed |= ui.add(egui::DragValue::new(&mut t.legs)).changed(); });
        if changed { self.arena_modified = true; }
    }
}
