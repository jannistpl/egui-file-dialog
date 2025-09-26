use egui_file_dialog::{FileDialog, OpeningMode};
use std::path::PathBuf;

use eframe::egui;

struct MyApp {
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    remember_pick: bool,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
            remember_pick: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Picked file").clicked() {
                let cfg = self.file_dialog.config_mut();
                cfg.opening_mode = OpeningMode::LastVisitedDir;
                if self.remember_pick {
                    if let Some(picked_file) = &self.picked_file {
                        cfg.initial_path = picked_file.clone();
                        cfg.opening_mode = OpeningMode::AlwaysInitialPath;
                    }
                }
                self.file_dialog.pick_file();
            }

            ui.checkbox(&mut self.remember_pick, "Remember last pick");

            ui.label(format!("Picked file: {:?}", self.picked_file));

            if let Some(path) = self.file_dialog.update(ctx).picked() {
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
