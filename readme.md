# Barcode Scanner Project

This project is a Rust-based application designed to interface with barcode scanners via serial ports. It features both a Command-Line Interface (CLI) and a Graphical User Interface (GUI) built with `egui` and `eframe`.

## Features

- **CLI Support:** Fully functional for single-port barcode scanning.
- **Single Port GUI:** Completed and functional.
- **Multiple Port Support:** Implemented but pending final push.
- **Split Code Structure:** Modular design with separate files for better maintainability.

## Project Structure

```
├── src
│   ├── main.rs       # Entry point of the application
│   ├── app.rs        # Handles the GUI application logic
│   └── scanner.rs    # Manages barcode scanner interactions
└── README.md         # Project documentation
```

## Getting Started

### Prerequisites
- Rust (recommended version: latest stable)
- Cargo for managing dependencies
- make sure switch mode kbw to virtual com or HID to communication

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/barcode-scanner.git
   cd barcode-scanner
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Usage

### CLI Mode
```bash
cargo run -- --port COM3
```

### GUI Mode
```bash
cargo run
```
- **Port Selection Dropdown:** Choose the correct COM port.
- **Live Barcode Display:** View scanned barcodes in real-time.
- **Auto-Detect Button:** Automatically detect available COM ports.

## TODO

- [x] CLI support for single-port scanning
- [x] GUI for single-port scanning
- [x] Multiple port support (implemented, pending push)
- [x] Code split for better maintainability
- [ ] Finalize and push multiple port feature

## License

This project is licensed under the MIT License.

---

*Developed with Rust and a lot of caffeine!*

