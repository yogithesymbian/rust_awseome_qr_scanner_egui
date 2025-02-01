
// src/app.rs

use crate::scanner::BarcodeScanner;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct BarcodeApp {
    entry_scanner: Option<Arc<Mutex<BarcodeScanner>>>,
    exit_scanner: Option<Arc<Mutex<BarcodeScanner>>>,
    available_ports: Vec<String>,
    entry_port: Option<String>,
    exit_port: Option<String>,
    scanned_data: Vec<String>,
    is_entry_dropdown_open: bool,
    is_exit_dropdown_open: bool,
}

impl Default for BarcodeApp {
    fn default() -> Self {
        Self {
            entry_scanner: None,
            exit_scanner: None,
            available_ports: BarcodeScanner::list_ports(),
            entry_port: None,
            exit_port: None,
            scanned_data: Vec::new(),
            is_entry_dropdown_open: false,
            is_exit_dropdown_open: false,
        }
    }
}

impl eframe::App for BarcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Barcode Scanner App");

            ui.horizontal(|ui| {
                ui.label("Entry Port:");
                if ui.button(&self.entry_port.clone().unwrap_or_else(|| "Select Port".into())).clicked() {
                    self.is_entry_dropdown_open = !self.is_entry_dropdown_open;
                    self.is_exit_dropdown_open = false;
                }
            });

            if self.is_entry_dropdown_open {
                egui::ComboBox::from_id_source("entry_port_dropdown")
                    .selected_text(self.entry_port.clone().unwrap_or_else(|| "Select Port".into()))
                    .show_ui(ui, |ui| {
                        for port in &self.available_ports {
                            if ui.selectable_label(self.entry_port.as_ref() == Some(port), port).clicked() {
                                self.entry_port = Some(port.clone());
                                self.entry_scanner = Some(Arc::new(Mutex::new(BarcodeScanner::new(port))));
                                self.is_entry_dropdown_open = false;
                            }
                        }
                    });
            }

            ui.horizontal(|ui| {
                ui.label("Exit Port:");
                if ui.button(&self.exit_port.clone().unwrap_or_else(|| "Select Port".into())).clicked() {
                    self.is_exit_dropdown_open = !self.is_exit_dropdown_open;
                    self.is_entry_dropdown_open = false;
                }
            });

            if self.is_exit_dropdown_open {
                egui::ComboBox::from_id_source("exit_port_dropdown")
                    .selected_text(self.exit_port.clone().unwrap_or_else(|| "Select Port".into()))
                    .show_ui(ui, |ui| {
                        for port in &self.available_ports {
                            if ui.selectable_label(self.exit_port.as_ref() == Some(port), port).clicked() {
                                self.exit_port = Some(port.clone());
                                self.exit_scanner = Some(Arc::new(Mutex::new(BarcodeScanner::new(port))));
                                self.is_exit_dropdown_open = false;
                            }
                        }
                    });
            }

            ui.separator();
            ui.heading("Scanned Barcodes:");
            for barcode in &self.scanned_data {
                ui.label(barcode);
            }
        });
    }
}
