use crate::app::Myapp;
use eframe::egui;

pub fn render(app: &mut Myapp, ui: &mut egui::Ui) {
    app.show_log_window = true;
    ui.separator();
}
