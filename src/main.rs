use eframe::egui;
use serialport::SerialPort;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Default)]
struct BarcodeApp {
    entry_port: Option<String>,
    exit_port: Option<String>,
    available_ports: Vec<String>,
    shared_state: Arc<Mutex<(Option<String>, Option<String>)>>,
}

impl BarcodeApp {
    fn new(shared_state: Arc<Mutex<(Option<String>, Option<String>)>>) -> Self {
        let ports = serialport::available_ports()
            .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
            .unwrap_or_else(|_| vec![]);

        Self {
            entry_port: None,
            exit_port: None,
            available_ports: ports,
            shared_state,
        }
    }
}

impl eframe::App for BarcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select Serial Ports");

            ui.horizontal(|ui| {
                ui.label("Entry Port:");
                egui::ComboBox::new("entry_port", "Select Entry Port")
                    .selected_text(
                        self.entry_port
                            .clone()
                            .unwrap_or_else(|| "Select...".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for port in &self.available_ports {
                            if ui
                                .selectable_label(self.entry_port.as_deref() == Some(port), port)
                                .clicked()
                            {
                                self.entry_port = Some(port.clone());
                                self.shared_state.lock().unwrap().0 = Some(port.clone());
                            }
                        }
                    });
                if self.entry_port.is_some() {
                    if ui.button("❌").clicked() {
                        self.entry_port = None;
                        self.shared_state.lock().unwrap().0 = None;
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Exit Port:");
                egui::ComboBox::new("exit_port", "Select Exit Port")
                    .selected_text(
                        self.exit_port
                            .clone()
                            .unwrap_or_else(|| "Select...".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for port in &self.available_ports {
                            if ui
                                .selectable_label(self.exit_port.as_deref() == Some(port), port)
                                .clicked()
                            {
                                self.exit_port = Some(port.clone());
                                self.shared_state.lock().unwrap().1 = Some(port.clone());
                            }
                        }
                    });
                if self.exit_port.is_some() {
                    if ui.button("❌").clicked() {
                        self.exit_port = None;
                        self.shared_state.lock().unwrap().1 = None;
                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() {
    let shared_state = Arc::new(Mutex::new((None, None)));
    let state_clone = Arc::clone(&shared_state);

    thread::spawn(move || {
        let mut last_entry = None;
        let mut last_exit = None;

        loop {
            let (entry, exit) = state_clone.lock().unwrap().clone();

            // If a port has changed, attempt to connect
            if entry != last_entry {
                last_entry = entry.clone();
                if let Some(port_name) = &entry {
                    if let Ok(mut port) = serialport::new(port_name, 9600).open() {
                        println!("Connected to entry port: {}", port_name);
                        listen_on_port(port.as_mut(), "Entry");
                    } else {
                        println!("Failed to connect to entry port: {}", port_name);
                    }
                }
            }

            if exit != last_exit {
                last_exit = exit.clone();
                if let Some(port_name) = &exit {
                    if let Ok(mut port) = serialport::new(port_name, 9600).open() {
                        println!("Connected to exit port: {}", port_name);
                        listen_on_port(port.as_mut(), "Exit");
                    } else {
                        println!("Failed to connect to exit port: {}", port_name);
                    }
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    let app = BarcodeApp::new(shared_state);
    eframe::run_native(
        "Barcode Scanner",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(app)),
    );
}

fn listen_on_port(port: &mut dyn SerialPort, port_type: &str) {
    let mut buffer = [0; 1024];

    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let barcode = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("[{}] Scanned barcode: {}", port_type, barcode);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                continue;
            }
            Err(e) => {
                println!("Error reading from {} port: {:?}", port_type, e);
                break;
            }
        }
    }
}
