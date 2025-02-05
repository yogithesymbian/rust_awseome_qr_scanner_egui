use eframe::egui;

pub struct MyApp {
    notifications: Vec<Notification>,
}

#[derive(Clone)]
pub struct Notification {
    title: String,
    message: String,
}

impl MyApp {
    // Global function to trigger a notification
    pub fn trigger_notification(&mut self, title: &str, message: &str) {
        let notification = Notification {
            title: title.to_string(),
            message: message.to_string(),
        };
        self.notifications.push(notification);
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            notifications: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Button to trigger a notification
            if ui.button("Trigger Notification").clicked() {
                self.trigger_notification("Notification Title", "This is the message body!");
            }
        });

        // Display all notifications at the bottom-right corner
        let screen_size = ctx.screen_rect().size();
        for (i, notification) in self.notifications.iter().enumerate() {
            let y_offset = 50.0 * (i as f32); // Space notifications vertically
            let notification_position = egui::Pos2 {
                x: screen_size.x - 200.0,           // Position from the right
                y: screen_size.y - 50.0 - y_offset, // Position from the bottom
            };

            egui::Area::new(format!("notification_{}", i))
                .anchor(egui::Align2::RIGHT_BOTTOM, notification_position.to_vec2()) // Convert Pos2 to Vec2
                .show(ctx, |ui| {
                    ui.group(|ui| {
                        ui.label(&notification.title);
                        ui.label(&notification.message);
                    });
                });
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Global Notification Example",
        eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(400.0, 200.0)),
            ..Default::default()
        },
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
