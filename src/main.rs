use eframe::egui;
use egui::ViewportBuilder;
use std::path::PathBuf;
use std::fs;
use orbit::keyid;
use mlua::Lua;
use std::sync::mpsc;

mod plugin_manager;
use plugin_manager::PluginManager;
mod lua_bindings;

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
    let mut popup = orbit::popup::PopupMessage::default();
    if let Err(_) = keyid(&mut popup) {
        popup.show_info("DRM check Error occurred.");
    }
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1152.0, 864.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Orbit",
        options,
        Box::new(move |cc| {
            let lua = Lua::new();
            let config_path = std::env::var("CONFIG").unwrap_or_default();
            lua.globals().set("CONFIG", config_path).unwrap_or_default();
            let (error_sender, error_receiver) = mpsc::channel();
            
            // Create Lua bindings with context
            if let Ok(exports) = lua_bindings::create_lua_module(&lua, cc.egui_ctx.clone(), error_sender.clone()) {
                lua.globals().set("orbit_egui", exports).unwrap_or_default();
            }
            
            Ok(Box::new(app))
        }),
    )
}

struct OrbitApp {
    popup: orbit::popup::PopupMessage,
    plugin_manager: PluginManager,
    lua: Lua,
    lua_error_receiver: mpsc::Receiver<String>,
    show_menu: bool,
    current_screen: Option<mlua::RegistryKey>,
}

impl Default for OrbitApp {
    fn default() -> Self {
        let lua = Lua::new();
        let (error_sender, error_receiver) = mpsc::channel();
        
        Self {
            popup: orbit::popup::PopupMessage::default(),
            plugin_manager: PluginManager::default(),
            lua,
            lua_error_receiver: error_receiver,
            show_menu: true,
            current_screen: None,
        }
    }
}

impl eframe::App for OrbitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set up Lua UI bindings with current context
        let (error_sender, _) = mpsc::channel();
        if let Ok(exports) = lua_bindings::create_lua_module(&self.lua, ctx.clone(), error_sender) {
            self.lua.globals().set("orbit_egui", exports).unwrap_or_default();
        }

        if let Ok(error) = self.lua_error_receiver.try_recv() {
            self.popup.show_error(error);
        }

        if self.show_menu {
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
                            let config_path = std::env::var("CONFIG").unwrap_or_default();
                            let index_path = PathBuf::from(config_path).join("plugins").join("game").join("index.lua");
                            
                            let lua_code = fs::read_to_string(&index_path).unwrap_or_else(|_| {
                                self.popup.show_error(format!("Failed to read Lua file at: {:?}", index_path));
                                String::new() // Return an empty string if reading fails
                            });

                            if !lua_code.is_empty() {
                                let lua_result = self.lua.load(&lua_code);
match lua_result.into_function() {
    Ok(chunk) => {
        match chunk.call::<_, mlua::Table>(()) {
            Ok(screen) => {
                self.current_screen = Some(self.lua.create_registry_value(screen).unwrap());
                self.show_menu = false;
            },
            Err(e) => {
                self.popup.show_error(format!("Error executing Lua chunk: {}", e));
            }
        }
    },
    Err(e) => {
        self.popup.show_error(format!("Error loading Lua chunk: {}", e));
    }
}
                            } else {
                                self.popup.show_error("Lua code is empty, cannot proceed.");
                            }
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
        } else if let Some(screen_key) = &self.current_screen {
            // Draw the current Lua screen
            if let Ok(screen) = self.lua.registry_value::<mlua::Table>(screen_key) {
                if let Ok(show) = screen.get::<_, mlua::Function>("show") {
                    if let Err(e) = show.call::<_, ()>(screen) {
                        self.popup.show_error(format!("Lua error: {}", e));
                        self.show_menu = true; // Return to menu on error
                    }
                }
            }
        }
    }
}
