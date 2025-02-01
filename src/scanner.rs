
// scanner.rs

use eframe::egui;
use serialport::SerialPort;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct BarcodeScanner {
    label: String,
    ports: Vec<String>,
    selected_port: Option<String>,
    barcode: Arc<Mutex<String>>,
}

impl BarcodeScanner {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            ports: serialport::available_ports()
                .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
                .unwrap_or_else(|_| vec![]),
            selected_port: None,
            barcode: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading(&self.label);

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

            if ui.button("Auto-Detect Ports").clicked() {
                self.ports = serialport::available_ports()
                    .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
                    .unwrap_or_else(|_| vec![]);
            }

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

            let barcode = self.barcode.lock().unwrap().clone();
            ui.label(format!("Scanned Barcode: {}", barcode));
        });
    }
}
