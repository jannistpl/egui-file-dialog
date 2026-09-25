use eframe::egui;
use egui_file_dialog::{FileDialog, FileDialogConfig};
use std::path::PathBuf;

struct MyApp {
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_dialog: FileDialog::with_config(FileDialogConfig {
                use_egui_modal: true,
                title: Some("Pick a file".to_string()),
                // anchor: Some((Align2::RIGHT_BOTTOM, Vec2 { x: -10., y: -10. })),
                modal_overlay_color: egui::Color32::from_rgba_premultiplied(0, 0, 100, 50),
                ..Default::default()
            }),
            picked_file: None,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Pick a file").highlight().clicked() {
                self.file_dialog.pick_file();
            }
            ui.add_space(10.);
            ui.label(format!("Picked file: {:?}", self.picked_file));

            if let Some(path) = self.file_dialog.update(ui).picked() {
                self.picked_file = Some(path.to_path_buf());
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "File dialog example",
        eframe::NativeOptions::default(),
        Box::new(|ctx| Ok(Box::new(MyApp::new(ctx)))),
    )
}
