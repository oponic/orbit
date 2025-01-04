use eframe::egui;
use std::path::PathBuf;
use std::fs;
use quick_xml::reader::Reader;
use quick_xml::events::Event;
enum Action {
    DeletePlugin(usize),
    ConfirmDelete(String),
    CancelDelete,
    TogglePlugin(usize),
}
pub struct PluginManager {
    pub show: bool,
    plugins: Vec<Plugin>,
    config_dir: PathBuf,
    selected_plugin: Option<usize>,
    confirm_delete: Option<String>,
}
struct Plugin {
    name: String,
    display_name: String,
    description: String,
    logo_path: Option<PathBuf>,
    path: PathBuf,
    enabled: bool,
    texture: Option<egui::TextureHandle>,
}
impl Plugin {
    fn load_metadata(path: &PathBuf) -> (String, String, Option<PathBuf>) {
        let index_path = path.join("index.xml");
        let mut display_name = String::new();
        let mut description = String::new();
        let mut logo_path = None;
        if let Ok(xml_content) = fs::read_to_string(index_path) {
            let mut reader = Reader::from_str(&xml_content);
            reader.trim_text(true);
            
            let mut buf = Vec::new();
            let mut current_element = String::new();
            
            loop {
                match reader.read_event_into(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    },
                    Ok(Event::Text(ref e)) => {
                        let text = e.unescape().unwrap_or_default().to_string();
                        match current_element.as_str() {
                            "name" => display_name = text,
                            "description" => description = text,
                            "logo" => logo_path = Some(PathBuf::from(text)),
                            _ => {}
                        }
                    },
                    Ok(Event::Eof) => break,
                    Err(_) => break,
                    _ => {}
                }
            }
        }
        
        (
            display_name.trim().to_string(),
            description.trim().to_string(),
            logo_path
        )
    }
    fn load_logo(&mut self, ctx: &egui::Context) {
        if self.texture.is_none() && self.logo_path.is_some() {
            let logo_path = self.path.join(self.logo_path.as_ref().unwrap());
            if let Ok(image_data) = fs::read(&logo_path) {
                if let Ok(image) = image::load_from_memory(&image_data) {
                    let size = [image.width() as _, image.height() as _];
                    let image_buffer = image.to_rgba8();
                    let pixels = image_buffer.as_flat_samples();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        size,
                        pixels.as_slice(),
                    );
                    self.texture = Some(ctx.load_texture(
                        &self.name,
                        color_image,
                        egui::TextureOptions::default(),
                    ));
                }
            }
        }
    }
}
impl Default for PluginManager {
    fn default() -> Self {
        let config_dir = PathBuf::from(std::env::var("CONFIG").unwrap_or_default());
        Self {
            show: false,
            plugins: Vec::new(),
            config_dir,
            selected_plugin: None,
            confirm_delete: None,
        }
    }
}
impl PluginManager {
    pub fn refresh_plugins(&mut self) {
        self.plugins.clear();
        let plugins_dir = self.config_dir.join("plugins");
        let wastebasket_dir = self.config_dir.join("wastebasket");
        
        if let Err(_) = fs::create_dir_all(&plugins_dir) {
            return;
        }
        if let Err(_) = fs::create_dir_all(&wastebasket_dir) {
            return;
        }
        if let Ok(entries) = fs::read_dir(&plugins_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(name) = entry.path().file_name() {
                        let name = name.to_string_lossy().into_owned();
                        let (display_name, description, logo_path) = Plugin::load_metadata(&entry.path());
                        self.plugins.push(Plugin {
                            name: name.clone(),
                            display_name: if display_name.is_empty() { name } else { display_name },
                            description,
                            logo_path,
                            path: entry.path(),
                            enabled: true,
                            texture: None,
                        });
                    }
                }
            }
        }
        if let Ok(entries) = fs::read_dir(&wastebasket_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(name) = entry.path().file_name() {
                        let name = name.to_string_lossy().into_owned();
                        let (display_name, description, logo_path) = Plugin::load_metadata(&entry.path());
                        self.plugins.push(Plugin {
                            name: name.clone(),
                            display_name: if display_name.is_empty() { name } else { display_name },
                            description,
                            logo_path,
                            path: entry.path(),
                            enabled: false,
                            texture: None,
                        });
                    }
                }
            }
        }
    }
    fn toggle_plugin(&mut self, index: usize) {
        if let Some(plugin) = self.plugins.get_mut(index) {
            let plugins_dir = self.config_dir.join("plugins");
            let wastebasket_dir = self.config_dir.join("wastebasket");
            
            let target_dir = if plugin.enabled {
                &wastebasket_dir
            } else {
                &plugins_dir
            };

            let new_path = target_dir.join(&plugin.name);
            if let Ok(_) = fs::rename(&plugin.path, &new_path) {
                plugin.enabled = !plugin.enabled;
                plugin.path = new_path;
            }
        }
    }
    fn delete_plugin(&mut self, index: usize) {
        if let Some(plugin) = self.plugins.get(index) {
            if let Ok(_) = fs::remove_dir_all(&plugin.path) {
                self.plugins.remove(index);
            }
        }
        self.confirm_delete = None;
    }
    pub fn draw(&mut self, ctx: &egui::Context) {
        if !self.show {
            return;
        }
        let mut pending_actions: Vec<Action> = Vec::new();
        egui::Window::new("Plugin Manager")
            .resizable(true)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Refresh").clicked() {
                        self.refresh_plugins();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("✕").clicked() {
                            self.show = false;
                        }
                    });
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, plugin) in self.plugins.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            plugin.load_logo(ctx);
                            if let Some(texture) = &plugin.texture {
                                let size = 48.0;
                                ui.add(egui::Image::new(texture).fit_to_exact_size(egui::vec2(size, size)));
                                ui.add_space(10.0);
                            }
                            ui.vertical(|ui| {
                                ui.heading(&plugin.display_name);
                                ui.label(&plugin.description);
                                
                                ui.horizontal(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if self.confirm_delete.as_ref() == Some(&plugin.name) {
                                            ui.horizontal(|ui| {
                                                if ui.button("Cancel").clicked() {
                                                    pending_actions.push(Action::CancelDelete);
                                                }
                                                if ui.button(egui::RichText::new("Confirm Delete").color(egui::Color32::RED)).clicked() {
                                                    pending_actions.push(Action::DeletePlugin(index));
                                                }
                                            });
                                        } else {
                                            if ui.button("Delete").clicked() {
                                                pending_actions.push(Action::ConfirmDelete(plugin.name.clone()));
                                            }
                                        }
                                        
                                        let toggle_text = if plugin.enabled { "Disable" } else { "Enable" };
                                        if ui.button(toggle_text).clicked() {
                                            pending_actions.push(Action::TogglePlugin(index));
                                        }
                                    });
                                });
                            });
                        });
                        ui.separator();
                    }
                });
            });
        for action in pending_actions {
            match action {
                Action::DeletePlugin(idx) => self.delete_plugin(idx),
                Action::ConfirmDelete(name) => self.confirm_delete = Some(name),
                Action::CancelDelete => self.confirm_delete = None,
                Action::TogglePlugin(idx) => self.toggle_plugin(idx),
            }
        }
    }
}
