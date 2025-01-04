use eframe::egui;
use egui::ViewportBuilder;
use std::path::PathBuf;
use std::fs;
use orbit::keyid;

mod popup;
use popup::PopupMessage;
mod plugin_manager;
use plugin_manager::PluginManager;

fn main() -> eframe::Result<()> {
    let mut app = OrbitApp::default();
    let config_dir = if cfg!(windows) {
        let mut path = PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default());
        path.push("Documents");
        path.push("Orbit");
        path
    } else {
        let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_default());
        path.push(".config");
        path.push("Orbit");
        path
    };
    std::env::set_var("CONFIG", config_dir.to_str().unwrap());
    if let Err(e) = fs::create_dir_all(&config_dir) {
        app.popup.show_warning(format!("Failed to create config directory: {}", e));
    }
    if let Err(e) = keyid() {
        app.popup.show_info(format!("DRM check Error: {}", e)); // make sure to finish DRM stuff and set this to panic after
    }
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1152.0, 864.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Orbit",
        options,
        Box::new(move |_cc| Ok(Box::new(app))),
    )
}
struct OrbitApp {
    popup: PopupMessage,
    plugin_manager: PluginManager,
}
impl Default for OrbitApp {
    fn default() -> Self {
        Self {
            popup: PopupMessage::default(),
            plugin_manager: PluginManager::default(),
        }
    }
}
impl eframe::App for OrbitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.popup.draw(ctx);
        self.plugin_manager.draw(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();
            let available_height = ui.available_height();
            let button_height = (available_height * 0.1).min(60.0);
            let button_width = (available_width * 0.3).min(200.0);
            let button_size = egui::vec2(button_width, button_height);
            let heading_size = (available_height * 0.08).min(42.0);
            let button_text_size = (available_height * 0.05).min(32.0);
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Orbit").size(heading_size));
            });
            ui.add_space(available_height * 0.1);
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.vertical_centered(|ui| {
                    if ui.add_sized(button_size, egui::Button::new(
                        egui::RichText::new("Start").size(button_text_size)
                    ).rounding(20.0)).clicked() {
                        self.popup.show_info("Starting application...");
                    }
                    ui.add_space(20.0);
                    if ui.add_sized(button_size, egui::Button::new(
                        egui::RichText::new("Plugins").size(button_text_size)
                    ).rounding(20.0)).clicked() {
                        self.plugin_manager.show = true;
                        self.plugin_manager.refresh_plugins();
                    }
                    ui.add_space(20.0);
                    if ui.add_sized(button_size, egui::Button::new(
                        egui::RichText::new("Quit").size(button_text_size)
                    ).rounding(20.0)).clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });
    }
}
