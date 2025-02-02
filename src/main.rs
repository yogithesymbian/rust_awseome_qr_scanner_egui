mod app;
mod scanner;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Barcode Scanner App",
        options,
        Box::new(|_cc| Box::new(app::BarcodeApp::default())),
    )
    .unwrap();
}
