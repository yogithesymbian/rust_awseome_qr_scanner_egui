use eframe::egui;
use tray_item::{IconSource, TrayItem};

fn main() {
    // Create a system tray item
    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("default-icon")).unwrap();

    // Add a menu item to the tray
    tray.add_label("Tray Label").unwrap();

    // Create a channel to communicate between the tray and the GUI
    let (tx, rx) = std::sync::mpsc::channel();

    // Add a menu item to show the GUI
    tray.add_menu_item("Show GUI", move || {
        tx.send(TrayEvent::ShowGui).unwrap();
    })
    .unwrap();

    // Add a menu item to quit the application
    tray.add_menu_item("Quit", move || {
        tx.send(TrayEvent::Quit).unwrap();
    })
    .unwrap();

    // Run the GUI in a separate thread
    std::thread::spawn(move || {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(300.0, 200.0)),
            ..Default::default()
        };

        eframe::run_native(
            "My egui App",
            options,
            Box::new(|cc| Box::new(MyApp::new(cc))),
        );
    });

    // Handle tray events
    for event in rx {
        match event {
            TrayEvent::ShowGui => {
                // Show the GUI window
                // This is a placeholder, you would need to implement a way to show/hide the window
                println!("Show GUI");
            }
            TrayEvent::Quit => {
                // Quit the application
                std::process::exit(0);
            }
        }
    }
}

enum TrayEvent {
    ShowGui,
    Quit,
}

struct MyApp {
    label: String,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            label: "Hello, world!".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui App");
            ui.label(&self.label);
        });
    }
}
