// main.rs

mod app;

use app::BarcodeApp;
use eframe::NativeOptions;

fn main() {
    let app = BarcodeApp::default();
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Barcode Scanner GUI",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}