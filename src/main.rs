use eframe::egui;
use serialport::available_ports;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn filter_scanned_data(data: String) -> String {
    // Remove Carriage Return (CR) and Line Feed (LF) characters from the end of the data
    data.trim_end_matches(|c| c == '\r' || c == '\n')
        .to_string()
}

struct BarcodeApp {
    ports: Vec<String>,
    selected_port_enter: Option<String>,
    selected_port_out: Option<String>,
    scan_results: Arc<Mutex<Vec<String>>>,
}

impl BarcodeApp {
    fn new() -> Self {
        let ports = available_ports()
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .map(|p| p.port_name)
            .collect();

        Self {
            ports,
            selected_port_enter: None,
            selected_port_out: None,
            scan_results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn start_scanning(&self, port_name: String) {
        let scan_results = Arc::clone(&self.scan_results);
        thread::spawn(move || {
            if let Ok(mut port) = serialport::new(&port_name, 9600)
                .timeout(Duration::from_secs(1))
                .open()
            {
                let mut buf = vec![0; 256];
                loop {
                    match port.read(&mut buf) {
                        Ok(bytes_read) => {
                            let data = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
                            let filtered_data = filter_scanned_data(data);
                            scan_results
                                .lock()
                                .unwrap()
                                .push(format!("{}: {}", port_name, filtered_data));
                        }
                        Err(_) => {}
                    }
                }
            }
        });
    }
}

impl eframe::App for BarcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Barcode Scanner");

            ui.label("Select Barcode Scanner for Enter Area:");
            egui::ComboBox::from_label("Enter Scanner")
                .selected_text(
                    self.selected_port_enter
                        .clone()
                        .unwrap_or_else(|| "None".to_string()),
                )
                .show_ui(ui, |ui| {
                    for port in &self.ports {
                        if ui
                            .selectable_label(self.selected_port_enter.as_ref() == Some(port), port)
                            .clicked()
                        {
                            self.selected_port_enter = Some(port.clone());
                            self.start_scanning(port.clone());
                        }
                    }
                });

            ui.label("Select Barcode Scanner for Out Area:");
            egui::ComboBox::from_label("Out Scanner")
                .selected_text(
                    self.selected_port_out
                        .clone()
                        .unwrap_or_else(|| "None".to_string()),
                )
                .show_ui(ui, |ui| {
                    for port in &self.ports {
                        if ui
                            .selectable_label(self.selected_port_out.as_ref() == Some(port), port)
                            .clicked()
                        {
                            self.selected_port_out = Some(port.clone());
                            self.start_scanning(port.clone());
                        }
                    }
                });

            ui.separator();
            ui.heading("Scan Results:");
            for result in self.scan_results.lock().unwrap().iter() {
                ui.label(result);
            }
        });

        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Barcode Scanner",
        options,
        Box::new(|_cc| Box::new(BarcodeApp::new())),
    );
}
