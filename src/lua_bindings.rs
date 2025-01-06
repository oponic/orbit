use mlua::prelude::*;
use eframe::egui;
use std::sync::mpsc;

pub struct LuaError(pub String);

pub struct LuaUi {
    ctx: egui::Context,
    error_sender: mpsc::Sender<String>,
}

impl LuaUserData for LuaUi {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // Basic UI Elements
        methods.add_method("button", |_lua, this, text: String| {
            let mut clicked = false;
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                clicked = ui.button(text).clicked();
            });
            Ok(clicked)
        });

        methods.add_method("label", |_lua, this, text: String| {
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.label(text);
            });
            Ok(())
        });

        // Layout methods
        methods.add_method("vertical", |lua, this, callback: LuaFunction| {
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.vertical(|ui| {
                    let temp_ui = LuaUi {
                        ctx: this.ctx.clone(),
                        error_sender: this.error_sender.clone(),
                    };
                    let lua_ui = lua.create_userdata(temp_ui).unwrap();
                    if let Err(e) = callback.call::<_, ()>(lua_ui) {
                        this.error_sender.send(format!("Lua error in vertical: {}", e)).unwrap_or_default();
                    }
                });
            });
            Ok(())
        });

        methods.add_method("horizontal", |lua, this, callback: LuaFunction| {
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.horizontal(|ui| {
                    let temp_ui = LuaUi {
                        ctx: this.ctx.clone(),
                        error_sender: this.error_sender.clone(),
                    };
                    let lua_ui = lua.create_userdata(temp_ui).unwrap();
                    if let Err(e) = callback.call::<_, ()>(lua_ui) {
                        this.error_sender.send(format!("Lua error in horizontal: {}", e)).unwrap_or_default();
                    }
                });
            });
            Ok(())
        });

        // Spacing and layout control
        methods.add_method("add_space", |_lua, this, amount: f32| {
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.add_space(amount);
            });
            Ok(())
        });

        // Text input
        methods.add_method_mut("text_edit", |_lua, this, (label, text): (String, String)| {
            let mut value = text.clone();
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.text_edit_singleline(&mut value);
                });
            });
            Ok(value)
        });

        // Checkbox
        methods.add_method_mut("checkbox", |_lua, this, (label, checked): (String, bool)| {
            let mut value = checked;
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.checkbox(&mut value, label);
            });
            Ok(value)
        });

        // Slider
        methods.add_method_mut("slider", |_lua, this, (label, value, min, max): (String, f64, f64, f64)| {
            let mut current_value = value;
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.add(egui::Slider::new(&mut current_value, min..=max).text(label));
            });
            Ok(current_value)
        });

        // Heading
        methods.add_method("heading", |_lua, this, text: String| {
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.heading(text);
            });
            Ok(())
        });

        // Color options for text
        methods.add_method("colored_text", |_lua, this, (text, r, g, b): (String, u8, u8, u8)| {
            egui::CentralPanel::default().show(&this.ctx, |ui| {
                ui.colored_label(egui::Color32::from_rgb(r, g, b), text);
            });
            Ok(())
        });
    }
}

pub fn create_lua_module(lua: &Lua, ctx: egui::Context, error_sender: mpsc::Sender<String>) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    
    // Create the main UI interface
    let lua_ui = LuaUi {
        ctx: ctx.clone(),
        error_sender: error_sender.clone(),
    };
    let lua_ui_userdata = lua.create_userdata(lua_ui)?;
    
    // Add the UI interface to the exports
    exports.set("ui", lua_ui_userdata)?;
    
    // Add any additional utility functions
    exports.set("version", "1.0.0")?;
    
    // Add a helper function to create colors
    exports.set("rgb", lua.create_function(|_, (r, g, b): (u8, u8, u8)| {
        Ok([r, g, b])
    })?)?;

    Ok(exports)
}