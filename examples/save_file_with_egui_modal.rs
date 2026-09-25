use egui_file_dialog::{FileDialog, FileDialogConfig};
use std::path::PathBuf;

struct MyApp {
    file_dialog: FileDialog,
    file_path: Option<PathBuf>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_dialog: FileDialog::with_config(FileDialogConfig {
                use_egui_modal: true,
                title: Some("Save as ...".to_string()),
                //anchor: Some((egui::Align2::RIGHT_BOTTOM, egui::Vec2 { x: -10., y: -10. })),
                modal_overlay_color: egui::Color32::from_rgba_premultiplied(0, 0, 100, 50),
                ..Default::default()
            }),
            file_path: None,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Save file").clicked() {
                self.file_dialog.save_file();
            }

            ui.label(format!("File to save: {:?}", self.file_path));

            if let Some(path) = self.file_dialog.update(ui).picked() {
                self.file_path = Some(path.to_path_buf());
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
