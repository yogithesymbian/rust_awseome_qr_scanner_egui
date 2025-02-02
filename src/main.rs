use eframe::egui;
use serialport::SerialPort;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Default)]
struct BarcodeApp {
    entry_port: Option<String>,
    exit_port: Option<String>,
    available_ports: Vec<String>,
    shared_state: Arc<Mutex<(Option<String>, Option<String>)>>,
    entry_thread: Option<thread::JoinHandle<()>>, // Track the entry port listener thread
    exit_thread: Option<thread::JoinHandle<()>>,  // Track the exit port listener thread
    entry_shutdown: Arc<AtomicBool>,              // Atomic flag to signal shutdown for entry thread
    exit_shutdown: Arc<AtomicBool>,               // Atomic flag to signal shutdown for exit thread
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
            entry_thread: None,
            exit_thread: None,
            entry_shutdown: Arc::new(AtomicBool::new(false)), // Initialize the shutdown flag
            exit_shutdown: Arc::new(AtomicBool::new(false)),  // Initialize the shutdown flag
        }
    }

    fn stop_listening(&mut self) {
        // Set the shutdown flag to true to signal threads to stop
        self.entry_shutdown.store(true, Ordering::SeqCst);
        self.exit_shutdown.store(true, Ordering::SeqCst);

        // Wait for threads to stop
        if let Some(thread) = self.entry_thread.take() {
            thread.join().unwrap();
        }
        if let Some(thread) = self.exit_thread.take() {
            thread.join().unwrap();
        }

        // Reset the shutdown flags for next connection attempt
        self.entry_shutdown.store(false, Ordering::SeqCst);
        self.exit_shutdown.store(false, Ordering::SeqCst);
    }
}

impl eframe::App for BarcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select Serial Ports");

            // Handle the entry port selection and listen logic
            ui.horizontal(|ui| {
                ui.label("Entry Port:");
                egui::ComboBox::new("entry_port", "Select Entry Port")
                    .selected_text(
                        self.entry_port
                            .clone()
                            .unwrap_or_else(|| "Select...".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        let mut selected_entry_port = None;

                        for port in &self.available_ports {
                            if ui
                                .selectable_label(self.entry_port.as_deref() == Some(port), port)
                                .clicked()
                            {
                                selected_entry_port = Some(port.clone());
                            }
                        }

                        if let Some(port) = selected_entry_port {
                            self.stop_listening();
                            self.entry_port = Some(port.clone());
                            self.shared_state.lock().unwrap().0 = Some(port.clone());

                            let entry_shutdown = self.entry_shutdown.clone(); // Clone the shutdown flag
                            let port_clone = port.clone();
                            self.entry_thread = Some(thread::spawn(move || {
                                listen_on_port(port_clone, "Entry", entry_shutdown);
                            }));
                        }
                    });

                if self.entry_port.is_some() {
                    if ui.button("❌").clicked() {
                        self.entry_port = None;
                        self.shared_state.lock().unwrap().0 = None;
                        self.stop_listening(); // Stop listening when disconnected
                    }
                }
            });

            // Handle the exit port selection and listen logic
            ui.horizontal(|ui| {
                ui.label("Exit Port:");
                egui::ComboBox::new("exit_port", "Select Exit Port")
                    .selected_text(
                        self.exit_port
                            .clone()
                            .unwrap_or_else(|| "Select...".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        let mut selected_exit_port = None;

                        for port in &self.available_ports {
                            if ui
                                .selectable_label(self.exit_port.as_deref() == Some(port), port)
                                .clicked()
                            {
                                selected_exit_port = Some(port.clone());
                            }
                        }

                        if let Some(port) = selected_exit_port {
                            self.stop_listening();
                            self.exit_port = Some(port.clone());
                            self.shared_state.lock().unwrap().1 = Some(port.clone());

                            let exit_shutdown = self.exit_shutdown.clone(); // Clone the shutdown flag
                            let port_clone = port.clone();
                            self.exit_thread = Some(thread::spawn(move || {
                                listen_on_port(port_clone, "Exit", exit_shutdown);
                            }));
                        }
                    });

                if self.exit_port.is_some() {
                    if ui.button("❌").clicked() {
                        self.exit_port = None;
                        self.shared_state.lock().unwrap().1 = None;
                        self.stop_listening(); // Stop listening when disconnected
                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() {
    let shared_state = Arc::new(Mutex::new((None::<String>, None::<String>)));

    let app = BarcodeApp::new(shared_state);
    eframe::run_native(
        "Barcode Scanner",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(app)),
    );
}

fn listen_on_port(port_name: String, port_type: &str, shutdown_flag: Arc<AtomicBool>) {
    match serialport::new(&port_name, 9600).open() {
        Ok(mut port) => {
            println!("Listening on {} port: {}", port_type, port_name);
            let mut buffer = [0; 1024];
            let mut barcode_data = String::new();

            loop {
                // Check if we should stop listening
                if shutdown_flag.load(Ordering::SeqCst) {
                    println!("Stop listening on port: {}", port_name);
                    break;
                }

                match port.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            let part = String::from_utf8_lossy(&buffer[..bytes_read]);
                            barcode_data.push_str(&part);

                            if barcode_data.ends_with('\n') || barcode_data.ends_with('\r') {
                                // let mut barcode = barcode.lock().unwrap();
                                // *barcode = barcode_data.trim().to_string();
                                println!("[{}] Scanned barcode: {}", port_type, barcode_data);
                                barcode_data.clear();
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        continue;
                    }
                    Err(e) => {
                        println!(
                            "Error reading from {} port {}: {:?}",
                            port_type, port_name, e
                        );
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!(
                "Failed to connect to {} port {}: {:?}",
                port_type, port_name, e
            );
        }
    }
}
