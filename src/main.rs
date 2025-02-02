use eframe::egui;
use serialport::available_ports;
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
        let ports = available_ports()
            .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
            .unwrap_or_else(|_| vec![]); // Fallback if no ports found

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

            // Entry Port Selection
            ui.horizontal(|ui| {
                ui.label("Entry Port:");
                egui::ComboBox::new("entry_port", "")
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

                // Clear button for entry port
                if self.entry_port.is_some() {
                    if ui.button("❌").clicked() {
                        self.entry_port = None;
                        self.shared_state.lock().unwrap().0 = None;
                    }
                }
            });

            // Exit Port Selection
            ui.horizontal(|ui| {
                ui.label("Exit Port:");
                egui::ComboBox::new("exit_port", "")
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

                // Clear button for exit port
                if self.exit_port.is_some() {
                    if ui.button("❌").clicked() {
                        self.exit_port = None;
                        self.shared_state.lock().unwrap().1 = None;
                    }
                }
            });
        });

        ctx.request_repaint(); // Ensure UI updates
    }
}

fn main() {
    let shared_state = Arc::new(Mutex::new((None, None)));

    let state_clone = Arc::clone(&shared_state); // ✅ Clone before moving

    // Background thread to monitor port changes
    thread::spawn(move || {
        let mut last_entry = None;
        let mut last_exit = None;

        loop {
            let (entry, exit) = state_clone.lock().unwrap().clone(); // ✅ Use cloned state
            if entry != last_entry || exit != last_exit {
                println!("Entry: {:?}, Exit: {:?}", entry, exit);
                last_entry = entry;
                last_exit = exit;
            }
            thread::sleep(Duration::from_millis(500)); // Polling interval
        }
    });

    let app = BarcodeApp::new(shared_state); // ✅ Original shared_state still available
    eframe::run_native(
        "Barcode Scanner",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(app)),
    );
}
