use std::io::{Read, Write};
use std::time::Duration;
use serialport::SerialPort;

fn main() {
    let port_name = "COM5"; // Change to match your system (e.g., "COM3" for Windows)

    let mut port = serialport::new(port_name, 9600)
        .timeout(Duration::from_secs(2)) // Increase timeout to wait for full barcode
        .open()
        .expect("Failed to open port");

    println!("Waiting for barcode scan...");

    let mut buffer = vec![0; 128];
    let mut barcode_data = String::new();

    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let part = String::from_utf8_lossy(&buffer[..bytes_read]);
                    barcode_data.push_str(&part);

                    // Check if scanner sends a newline or carriage return at the end
                    if barcode_data.ends_with('\n') || barcode_data.ends_with('\r') {
                        barcode_data = barcode_data.trim().to_string(); // Remove extra spaces/newlines
                        println!("Scanned Barcode: {}", barcode_data);
                        barcode_data.clear(); // Reset for next scan
                    }
                }
            }
            Err(e) => eprintln!("Error reading barcode: {:?}", e),
        }
    }
}
