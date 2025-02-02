use eframe::egui;

pub struct BarcodeApp {
    entry_port: Option<String>,
    exit_port: Option<String>,
    scanned_data: Vec<String>,
    is_entry_dropdown_open: bool,
    is_exit_dropdown_open: bool,
}

impl BarcodeApp {
    pub fn new() -> Self {
        Self {
            entry_port: None,
            exit_port: None,
            scanned_data: Vec::new(),
            is_entry_dropdown_open: false,
            is_exit_dropdown_open: false,
        }
    }

    fn handle_scanning(&mut self, barcode: &str) {
        if let Some(entry) = &self.entry_port {
            if let Some(exit) = &self.exit_port {
                // If both entry and exit ports are selected, show both
                if entry == exit {
                    self.scanned_data.push(format!("entry_exit: {}", barcode));
                } else {
                    self.scanned_data
                        .push(format!("entry: {} or exit: {}", entry, barcode));
                }
            } else {
                self.scanned_data.push(format!("entry: {}", entry));
            }
        } else if let Some(exit) = &self.exit_port {
            self.scanned_data.push(format!("exit: {}", exit));
        }
    }
}

impl eframe::App for BarcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Barcode Scanner");

            // Entry Port Section
            ui.horizontal(|ui| {
                ui.label("Entry Port:");
                if ui
                    .button(
                        self.entry_port
                            .clone()
                            .unwrap_or_else(|| "Select Port".to_string()),
                    )
                    .clicked()
                {
                    self.is_entry_dropdown_open = !self.is_entry_dropdown_open;
                    self.is_exit_dropdown_open = false;
                }

                if self.entry_port.is_some() {
                    if ui.button("Remove").clicked() {
                        self.entry_port = None;
                    }
                }
            });

            if self.is_entry_dropdown_open {
                egui::ComboBox::from_id_source("entry_port_dropdown")
                    .selected_text(
                        self.entry_port
                            .clone()
                            .unwrap_or_else(|| "Select Port".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for port in &["COM1", "COM2", "COM3", "COM4", "COM5", "COM6"] {
                            if ui
                                .selectable_label(self.entry_port.as_deref() == Some(port), *port)
                                .clicked()
                            {
                                self.entry_port = Some(port.to_string());
                                self.is_entry_dropdown_open = false;
                            }
                        }
                    });
            }

            // Exit Port Section
            ui.horizontal(|ui| {
                ui.label("Exit Port:");
                if ui
                    .button(
                        self.exit_port
                            .clone()
                            .unwrap_or_else(|| "Select Port".to_string()),
                    )
                    .clicked()
                {
                    self.is_exit_dropdown_open = !self.is_exit_dropdown_open;
                    self.is_entry_dropdown_open = false;
                }

                if self.exit_port.is_some() {
                    if ui.button("Remove").clicked() {
                        self.exit_port = None;
                    }
                }
            });

            if self.is_exit_dropdown_open {
                egui::ComboBox::from_id_source("exit_port_dropdown")
                    .selected_text(
                        self.entry_port
                            .clone()
                            .unwrap_or_else(|| "Select Port".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for port in &["COM1", "COM2", "COM3", "COM4", "COM5", "COM6"] {
                            if ui
                                .selectable_label(self.entry_port.as_deref() == Some(port), *port)
                                .clicked()
                            {
                                self.exit_port = Some(port.to_string());
                                self.is_exit_dropdown_open = false;
                            }
                        }
                    });
            }

            ui.separator();

            // Button to simulate barcode scanning
            if ui.button("Scan Barcode").clicked() {
                // Simulate scanning a barcode
                self.handle_scanning("12345");
            }

            // Display scanned barcodes
            ui.heading("Scanned Barcodes:");
            for barcode in &self.scanned_data {
                ui.label(barcode);
            }
        });
    }
}

fn main() {
    let app = BarcodeApp::new();
    eframe::run_native(
        "Barcode Scanner",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(app)),
    );
}
