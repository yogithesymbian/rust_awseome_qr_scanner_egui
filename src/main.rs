use eframe::{egui, App};
use serialport::SerialPort;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct BarcodeApp {
    ports: Vec<String>,
    selected_port: Option<String>,
    barcode: Arc<Mutex<String>>,
}

impl Default for BarcodeApp {
    fn default() -> Self {
        Self {
            ports: serialport::available_ports()
                .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
                .unwrap_or_else(|_| vec![]),
            selected_port: None,
            barcode: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl App for BarcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Barcode Scanner");

            // Port Selection Dropdown
            egui::ComboBox::from_label("Select Port")
                .selected_text(
                    self.selected_port
                        .as_ref()
                        .map_or("None", String::as_str),
                )
                .show_ui(ui, |combo| {
                    for port in &self.ports {
                        combo.selectable_value(
                            &mut self.selected_port,
                            Some(port.clone()),
                            port,
                        );
                    }
                });

            // Auto-Detect Button
            if ui.button("Auto-Detect Ports").clicked() {
                self.ports = serialport::available_ports()
                    .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
                    .unwrap_or_else(|_| vec![]);
            }

            // Start Reading Barcode
            if let Some(port_name) = &self.selected_port {
                let barcode = Arc::clone(&self.barcode);
                let port_name = port_name.clone();

                thread::spawn(move || {
                    if let Ok(mut port) = serialport::new(&port_name, 9600)
                        .timeout(Duration::from_millis(100))
                        .open()
                    {
                        let mut buffer = vec![0; 128];
                        let mut barcode_data = String::new();

                        loop {
                            if let Ok(bytes_read) = port.read(&mut buffer) {
                                if bytes_read > 0 {
                                    let part = String::from_utf8_lossy(&buffer[..bytes_read]);
                                    barcode_data.push_str(&part);

                                    if barcode_data.ends_with('\n') || barcode_data.ends_with('\r') {
                                        let mut barcode = barcode.lock().unwrap();
                                        *barcode = barcode_data.trim().to_string();
                                        barcode_data.clear();
                                    }
                                }
                            }
                        }
                    }
                });
            }

            // Live Barcode Display
            let barcode = self.barcode.lock().unwrap().clone();
            ui.label(format!("Scanned Barcode: {}", barcode));
        });
    }
}

fn main() {
    let app = BarcodeApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Barcode Scanner GUI",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}
