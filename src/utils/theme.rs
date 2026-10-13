//! Font and text-style installation, done once per egui context.
//!
//! `ctx.set_fonts` used to run every frame with `NOTO_FONT.to_vec()`: a 16 MB copy plus a 16 MB byte-wise
//! comparison inside egui (`set_fonts` checks "did the definitions change?") on every repaint, about 5 ms
//! of the ~6 ms a frame took (see the timings in the tests module).

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

fn flag(ctx: &egui::Context, name: &'static str) -> bool {
    ctx.data(|d| d.get_temp::<bool>(egui::Id::new(name))).unwrap_or(false)
}

fn set_flag(ctx: &egui::Context, name: &'static str) {
    ctx.data_mut(|d| d.insert_temp(egui::Id::new(name), true));
}

/// Puts NotoSansCJKjp first in every family, once. The font bytes are borrowed from the binary
/// (`FontData::from_static`), not copied.
pub fn install_fonts(ctx: &egui::Context) {
    const FLAG: &str = "mhf_file_editor::fonts_installed";
    if flag(ctx, FLAG) {
        return;
    }
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("noto".to_owned(), FontData::from_static(crate::NOTO_FONT));
    for family in [FontFamily::Proportional, FontFamily::Monospace, FontFamily::Name("noto".into())] {
        fonts.families.entry(family).or_default().insert(0, "noto".to_owned());
    }
    ctx.set_fonts(fonts);
    set_flag(ctx, FLAG);
}

/// The larger text sizes the mhfdat editor uses, once.
pub fn install_mhfdat_text_styles(ctx: &egui::Context) {
    const FLAG: &str = "mhf_file_editor::mhfdat_text_styles_installed";
    if flag(ctx, FLAG) {
        return;
    }
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(24.0, FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(18.0, FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(16.0, FontFamily::Monospace)),
        (egui::TextStyle::Button, egui::FontId::new(18.0, FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(14.0, FontFamily::Proportional)),
    ]
    .into();
    ctx.set_style(style);
    set_flag(ctx, FLAG);
}
