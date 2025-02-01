
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
                let mut buffer = [0; 1024];
                loop {
                    if let Ok(bytes_read) = port.read(&mut buffer) {
                        if bytes_read > 0 {
                            let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                            println!("Scanned Barcode from {}: {}", port_name, data);
                        }
                    }
                }
            }
        });
    }
} 
