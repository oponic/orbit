use eframe::egui;

pub enum PopupType {
    Error,
    Warning,
    Info,
    Panic,
}

pub struct PopupMessage {
    pub message: String,
    pub popup_type: PopupType,
    pub show: bool,
}

impl Default for PopupMessage {
    fn default() -> Self {
        Self {
            message: String::new(),
            popup_type: PopupType::Info,
            show: false,
        }
    }
}

impl PopupMessage {
    pub fn show_error(&mut self, message: impl Into<String>) {
        self.message = message.into();
        self.popup_type = PopupType::Error;
        self.show = true;
    }

    pub fn show_warning(&mut self, message: impl Into<String>) {
        self.message = message.into();
        self.popup_type = PopupType::Warning;
        self.show = true;
    }

    pub fn show_info(&mut self, message: impl Into<String>) {
        self.message = message.into();
        self.popup_type = PopupType::Info;
        self.show = true;
    }

    pub fn show_panic(&mut self, message: impl Into<String>) {
        self.message = message.into();
        self.popup_type = PopupType::Panic;
        self.show = true;
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.show {
                return;
            }

            let popup_id = egui::Id::new("popup_window");
            
            match self.popup_type {
                PopupType::Panic => {
                    egui::Window::new("Panic")
                        .id(popup_id)
                        .fixed_size(ctx.available_rect().size() * 0.8)
                        .movable(false)
                        .resizable(false)
                        .collapsible(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .show(ctx, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(20.0);
                                ui.heading(egui::RichText::new("Panic")
                                    .color(egui::Color32::RED)
                                    .size(32.0));
                                ui.add_space(20.0);
                                ui.label(
                                    egui::RichText::new(&self.message)
                                        .color(egui::Color32::RED)
                                        .size(16.0)
                                );
                                ui.add_space(20.0);
                                ui.label(
                                    egui::RichText::new("The application must be restarted.")
                                        .color(egui::Color32::RED)
                                        .size(16.0)
                                );
                                if ui.button(egui::RichText::new("Exit")
                                    .color(egui::Color32::RED)
                                    .size(20.0)).clicked() {
                                    std::process::exit(1);
                                }
                            });
                        });
                },
                _ => {
                    egui::Window::new("Message")
                        .id(popup_id)
                        .movable(true)
                        .resizable(false)
                        .collapsible(false)
                        .default_size([300.0, 100.0])
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                let (text_color, prefix) = match self.popup_type {
                                    PopupType::Error => (egui::Color32::RED, "Error: "),
                                    PopupType::Warning => (egui::Color32::YELLOW, "Warning: "),
                                    PopupType::Info => (egui::Color32::WHITE, "Info: "),
                                    PopupType::Panic => unreachable!(),
                                };

                                ui.colored_label(text_color, format!("{}{}", prefix, self.message));
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("OK").clicked() {
                                        self.show = false;
                                    }
                                });
                            });
                        });
                }
            }
        }));

        if let Err(e) = result {
            let error_msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown error occurred in popup system".to_string()
            };

            // Show native message box and exit
            rfd::MessageDialog::new()
                .set_title("Fatal Error")
                .set_description(&format!("A fatal error occurred in the popup system: {}", error_msg))
                .set_buttons(rfd::MessageButtons::Ok)
                .set_level(rfd::MessageLevel::Error)
                .show();

            std::process::exit(1);
        }
    }
}
