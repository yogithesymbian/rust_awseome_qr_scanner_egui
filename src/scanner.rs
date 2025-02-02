// src/scanner.rs
use serialport::SerialPort;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct BarcodeScanner {
    port_name: String,
    port_type: String, // Track whether it's "entry" or "exit"
}

impl BarcodeScanner {
    pub fn new(port_name: &str, port_type: &str) -> Self {
        let scanner = Self {
            port_name: port_name.to_string(),
            port_type: port_type.to_string(),
        };
        scanner.start_listening();
        scanner
    }

    pub fn list_ports() -> Vec<String> {
        serialport::available_ports()
            .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
            .unwrap_or_default()
    }

    fn start_listening(&self) {
        let port_name = self.port_name.clone();
        let port_type = self.port_type.clone(); // Capture port type (entry or exit)
        thread::spawn(move || {
            if let Ok(mut port) = serialport::new(&port_name, 9600).open() {
                let mut buffer = vec![0; 1024]; // Adjust the buffer size if needed
                let mut barcode_data = String::new(); // To accumulate barcode data

                loop {
                    if let Ok(bytes_read) = port.read(&mut buffer) {
                        if bytes_read > 0 {
                            // Convert bytes read into a part of the barcode data
                            let part = String::from_utf8_lossy(&buffer[..bytes_read]);
                            barcode_data.push_str(&part);
                            // Check if scanner sends a newline or carriage return at the end
                            if barcode_data.ends_with('\n') || barcode_data.ends_with('\r') {
                                barcode_data = barcode_data.trim().to_string(); // Remove extra spaces/newlines
                                println!(
                                    "[2] Scanned Barcode: from {} : {} {}",
                                    port_name, port_type, barcode_data
                                );
                                barcode_data.clear(); // Reset for next scan
                            }
                        }
                    }
                }
            }
        });
    }
}
