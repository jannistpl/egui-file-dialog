use std::path::PathBuf;

use eframe::egui;
use egui_file_dialog::FileDialog;

struct MyApp {
    file_dialog: FileDialog,
    selected_items: Option<Vec<PathBuf>>,
    mode: SelMode,
}

enum SelMode {
    Single,
    Multiple,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            file_dialog: FileDialog::new(),
            selected_items: None,
            mode: SelMode::Single,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Select single").clicked() {
                self.mode = SelMode::Single;
                self.file_dialog.select_file();
            }
            if ui.button("Select multiple").clicked() {
                self.mode = SelMode::Multiple;
                self.file_dialog.select_multiple();
            }

            ui.label("Selected items:");

            if let Some(items) = &self.selected_items {
                for item in items {
                    ui.label(format!("{:?}", item));
                }
            } else {
                ui.label("None");
            }

            self.file_dialog
                .update_with_right_panel_ui(ctx, &mut |ui, dia| match self.mode {
                    SelMode::Single => {
                        ui.heading("Active item");
                        ui.small(format!("{:#?}", dia.active_entry()));
                    }
                    SelMode::Multiple => {
                        ui.heading("Selected items");
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .max_height(ui.available_height())
                            .show(ui, |ui| {
                                for item in dia.active_selected_entries() {
                                    ui.small(format!("{item:#?}"));
                                    ui.separator();
                                }
                            });
                    }
                });

            match self.mode {
                SelMode::Single => {
                    if let Some(item) = self.file_dialog.take_selected() {
                        self.selected_items = Some(vec![item]);
                    }
                }
                SelMode::Multiple => {
                    if let Some(items) = self.file_dialog.take_selected_multiple() {
                        self.selected_items = Some(items);
                    }
                }
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
