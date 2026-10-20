use super::*;

impl MhfdatApp {
    /// Shop > Guild Shop: the items each hall of the Guild Shop sells for Guild Tickets, with their prices (header 0xC8,
    /// record 7). A list of another length is written again at the end of the file.
    pub(in crate::app) fn show_guild_shop(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "guild_shop");
        MhfdatApp::section_header(ui, "Guild Shop: items sold for Guild Tickets", |_| {});
        if self.guild_shop.is_empty() {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No Guild Shop found!");
            return;
        }
        ui.label("Each hall has its own list: the client uses Lv1 by default, Lv2, Lv Max, and the Diva event hall in its stage. The price is read at the same position as the item and paid with Guild Tickets (item 1026), from the item pouch first, then the item box. The file ends a list with a 0, so an item id is 1 or more.");
        self.guild_shop_hall = self.guild_shop_hall.min(self.guild_shop.len() - 1);
        ui.horizontal_wrapped(|ui| {
            for (i, h) in self.guild_shop.iter().enumerate() {
                if ui.selectable_label(self.guild_shop_hall == i, format!("{} ({})", h.name, h.items.rows.len())).clicked() {
                    self.guild_shop_hall = i;
                    self.guild_shop_page = 0;
                }
            }
        });
        ui.add_space(4.0);
        let selected = self.guild_shop_hall;
        let names = &self.item_names;
        let hall = &mut self.guild_shop[selected];
        if hall.items.offset == 0 || hall.prices.offset == 0 {
            ui.label("This hall has a null pointer: nothing to edit.");
            return;
        }
        if hall.items.rows.len() != hall.prices.rows.len() {
            ui.colored_label(egui::Color32::from_rgb(220, 70, 70), "The item list and the price list do not have the same length: nothing to edit.");
            return;
        }
        let mut changed = false;
        if ui.button("Add item").clicked() {
            let next = hall.items.rows.last().copied().unwrap_or(0).saturating_add(1).max(1);
            hall.items.rows.push(next);
            hall.prices.rows.push(1);
            self.guild_shop_page = ((hall.items.rows.len() - 1) / LIST_PAGE_SIZE) as u32;
            changed = true;
        }
        let total = hall.items.rows.len();
        let (start, end, total_pages) = crate::app::list_window(&mut self.guild_shop_page, total);
        let mut remove = None;
        MhfdatApp::entity_list(ui, "guild_shop_scroll", "Item", start, end, total, |ui| {
            egui::Grid::new(("guild_shop_rows", selected)).striped(true).spacing([10.0, 4.0]).show(ui, |ui| {
                ui.strong("#");
                ui.strong("Item");
                ui.strong("");
                ui.strong("Tickets");
                ui.strong("");
                ui.end_row();
                for i in start..end {
                    ui.label(i.to_string());
                    changed |= ui.add(egui::DragValue::new(&mut hall.items.rows[i]).clamp_range(1..=u16::MAX)).changed();
                    ui.label(MhfdatApp::item_label(names, hall.items.rows[i]));
                    changed |= ui.add(egui::DragValue::new(&mut hall.prices.rows[i])).changed();
                    if ui.small_button("Remove").clicked() {
                        remove = Some(i);
                    }
                    ui.end_row();
                }
            });
        });
        if let Some(i) = remove {
            hall.items.rows.remove(i);
            hall.prices.rows.remove(i);
            changed = true;
        }
        MhfdatApp::pagination_controls(ui, &mut self.guild_shop_page, total_pages);
        if changed {
            self.guild_shop_modified = true;
        }
    }
}
