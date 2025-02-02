// src/scanner.rs

use serialport::SerialPort;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct BarcodeScanner {
    port_name: String,
}

impl BarcodeScanner {
    pub fn new(port_name: &str) -> Self {
        let scanner = Self {
            port_name: port_name.to_string(),
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
                            println!("[1] Scanned Barcode: from {} : {}", port_name, barcode_data);
                            // Check if scanner sends a newline or carriage return at the end
                            if barcode_data.ends_with('\n') || barcode_data.ends_with('\r') {
                                barcode_data = barcode_data.trim().to_string(); // Remove extra spaces/newlines
                                println!(
                                    "[2] Scanned Barcode: from {} : {}",
                                    port_name, barcode_data
                                );
                                barcode_data.clear(); // Reset for next scan
                            }
                            println!("[3] Scanned Barcode: from {} : {}", port_name, barcode_data);
                            // scan_results
                            //     .lock()
                            //     .unwrap()
                            //     .push(format!("{}: {}", port_name, barcode_data));
                        }
                    }
                }
            }
        });
    }
}
