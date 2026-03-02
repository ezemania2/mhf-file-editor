use super::*;

const PAGE_SIZE: usize = 20;
const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn format_timestamp(ts: u32) -> String {
    if ts == 0 {
        return "None".to_string();
    }
    chrono::DateTime::from_timestamp(ts as i64, 0)
        .map(|dt| dt.format(DATE_FORMAT).to_string())
        .unwrap_or_else(|| format!("{}", ts))
}

fn parse_timestamp(s: &str) -> Option<u32> {
    chrono::NaiveDateTime::parse_from_str(s.trim(), DATE_FORMAT)
        .ok()
        .and_then(|dt| dt.and_utc().timestamp().try_into().ok())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EventTimeExport {
    idx: usize,
    event_id: u32,
    start_time: String,
    end_time: String,
}

impl MhfdatApp {
    pub fn show_seasonal_events_tab(&mut self, ui: &mut egui::Ui) {
        if !self.view_mode.contains_key("seasonal_events") {
            self.view_mode.insert("seasonal_events".to_string(), ViewMode::List);
        }

        match self.view_mode.get("seasonal_events").cloned().unwrap_or(ViewMode::List) {
            ViewMode::List => self.show_seasonal_events_list(ui),
            ViewMode::Details => self.show_seasonal_events_details(ui),
        }
    }

    fn show_seasonal_events_list(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, "Seasonal Event Timing", |ui| {
            if ui.button("Export to JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_filename("seasonal_events.json")
                    .show_save_single_file()
                {
                    let export: Vec<EventTimeExport> = self.seasonal_events.iter().enumerate()
                        .map(|(i, ev)| EventTimeExport {
                            idx: i,
                            event_id: ev.event_id,
                            start_time: format_timestamp(ev.start_time),
                            end_time: format_timestamp(ev.end_time),
                        })
                        .collect();
                    if let Ok(text) = serde_json::to_string_pretty(&export) {
                        let _ = std::fs::write(path.to_str().unwrap_or("seasonal_events.json"), text);
                    }
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file()
                {
                    if let Ok(data) = std::fs::read_to_string(path.to_str().unwrap_or("")) {
                        if let Ok(imported) = serde_json::from_str::<Vec<EventTimeExport>>(&data) {
                            self.seasonal_events = imported.into_iter().map(|e| {
                                crate::model::mhfdat::EventTime {
                                    event_id: e.event_id,
                                    start_time: parse_timestamp(&e.start_time).unwrap_or(0),
                                    end_time: parse_timestamp(&e.end_time).unwrap_or(0),
                                }
                            }).collect();
                            self.seasonal_events_modified = true;
                        }
                    }
                }
            }
        });
        ui.separator();

        let total = self.seasonal_events.len();
        if total == 0 {
            ui.label("No seasonal event data loaded.");
            return;
        }

        let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;

        ui.label(format!("Total events: {}", total));
        MhfdatApp::pagination_controls(ui, &mut self.seasonal_events_page, total_pages);
        ui.separator();

        let page = (self.seasonal_events_page as usize).min(total_pages.saturating_sub(1));
        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(total);

        let display_data: Vec<_> = self.seasonal_events[start..end]
            .iter()
            .enumerate()
            .map(|(i, ev)| {
                let idx = start + i;
                (idx, ev.event_id, format_timestamp(ev.start_time), format_timestamp(ev.end_time))
            })
            .collect();

        let scroll_height = ui.available_height() - 10.0;
        egui::ScrollArea::vertical()
            .max_height(scroll_height)
            .show(ui, |ui| {
                egui::Grid::new("seasonal_events_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Idx");
                        ui.label("Event ID");
                        ui.label("Start");
                        ui.label("End");
                        ui.end_row();

                        for (idx, event_id, start_str, end_str) in &display_data {
                            let selected = self.selected_seasonal_event_index == Some(*idx);
                            if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                                self.selected_seasonal_event_index = Some(*idx);
                                self.seasonal_event_last_edited_idx = None;
                                self.view_mode.insert("seasonal_events".to_string(), ViewMode::Details);
                            }
                            ui.label(format!("{}", event_id));
                            ui.label(start_str);
                            ui.label(end_str);
                            ui.end_row();
                        }
                    });
            });
    }

    fn show_seasonal_events_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("seasonal_events".to_string(), ViewMode::List);
            self.seasonal_event_last_edited_idx = None;
            return;
        }
        ui.separator();

        let Some(idx) = self.selected_seasonal_event_index else {
            ui.label("Select an event to edit.");
            return;
        };

        if idx >= self.seasonal_events.len() {
            ui.label("Selected index is out of range.");
            return;
        }

        // Initialize edit strings when switching to a new event
        if self.seasonal_event_last_edited_idx != Some(idx) {
            let ev = &self.seasonal_events[idx];
            self.seasonal_event_start_str = format_timestamp(ev.start_time);
            self.seasonal_event_end_str = format_timestamp(ev.end_time);
            self.seasonal_event_last_edited_idx = Some(idx);
        }

        ui.heading(format!("Seasonal Event #{}", idx));
        ui.separator();

        let mut event_id = self.seasonal_events[idx].event_id;
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Event ID:");
            if ui.add(egui::DragValue::new(&mut event_id).speed(1.0)).changed() {
                changed = true;
            }
        });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Start Time:");
            let start_valid = parse_timestamp(&self.seasonal_event_start_str).is_some()
                || self.seasonal_event_start_str.trim() == "None";
            let start_response = ui.add(
                egui::TextEdit::singleline(&mut self.seasonal_event_start_str)
                    .desired_width(250.0)
                    .text_color(if start_valid { egui::Color32::WHITE } else { egui::Color32::RED })
            );
            if start_response.changed() {
                if let Some(ts) = parse_timestamp(&self.seasonal_event_start_str) {
                    self.seasonal_events[idx].start_time = ts;
                    changed = true;
                } else if self.seasonal_event_start_str.trim() == "None" {
                    self.seasonal_events[idx].start_time = 0;
                    changed = true;
                }
            }
        });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("End Time:  ");
            let end_valid = parse_timestamp(&self.seasonal_event_end_str).is_some()
                || self.seasonal_event_end_str.trim() == "None";
            let end_response = ui.add(
                egui::TextEdit::singleline(&mut self.seasonal_event_end_str)
                    .desired_width(250.0)
                    .text_color(if end_valid { egui::Color32::WHITE } else { egui::Color32::RED })
            );
            if end_response.changed() {
                if let Some(ts) = parse_timestamp(&self.seasonal_event_end_str) {
                    self.seasonal_events[idx].end_time = ts;
                    changed = true;
                } else if self.seasonal_event_end_str.trim() == "None" {
                    self.seasonal_events[idx].end_time = 0;
                    changed = true;
                }
            }
        });

        ui.add_space(4.0);
        ui.separator();
        ui.label("Format: YYYY-MM-DD HH:MM:SS (or \"None\" for 0)");

        if changed {
            self.seasonal_events[idx].event_id = event_id;
            self.seasonal_events_modified = true;
        }
    }
}
