// app.rs

mod scanner;

use crate::scanner::BarcodeScanner;
use eframe::{egui, App};
use std::sync::{Arc, Mutex};

pub struct BarcodeApp {
    entry_scanner: BarcodeScanner,
    exit_scanner: BarcodeScanner,
}

impl Default for BarcodeApp {
    fn default() -> Self {
        Self {
            entry_scanner: BarcodeScanner::new("Entry"),
            exit_scanner: BarcodeScanner::new("Exit"),
        }
    }
}

impl App for BarcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Barcode Scanner GUI");

            ui.horizontal(|ui| {
                self.entry_scanner.ui(ui);
                self.exit_scanner.ui(ui);
            });
        });
    }
}
