use mlua::prelude::*;
use eframe::egui;
use std::sync::mpsc;

pub struct LuaError(pub String);

struct LuaUi {
    ui: *mut egui::Ui,
    error_sender: mpsc::Sender<String>,
}

impl LuaUserData for LuaUi {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("button", |_lua, this, text: String| {
            let ui = unsafe { &mut *this.ui };
            Ok(ui.button(text).clicked())
        });

        methods.add_method("label", |_lua, this, text: String| {
            let ui = unsafe { &mut *this.ui };
            ui.label(text);
            Ok(())
        });
    }
}

pub fn create_lua_module(lua: &Lua, error_sender: mpsc::Sender<String>) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    
    exports.set("vertical", lua.create_function(move |lua, callback: LuaFunction| {
        let ctx = egui::Context::default();
        let mut ui = egui::Ui::new(
            ctx.clone(),
            egui::Id::new("temp"),
            egui::UiBuilder::default(),
        );
        
        let lua_ui = LuaUi {
            ui: &mut ui as *mut egui::Ui,
            error_sender: error_sender.clone(),
        };
        let lua_ui_userdata = lua.create_userdata(lua_ui)?;
        
        if let Err(e) = callback.call::<_, ()>(lua_ui_userdata) {
            error_sender.send(format!("Lua error: {}", e)).unwrap_or_default();
        }
        Ok(())
    })?)?;

    exports.set("horizontal", lua.create_function(|lua, callback: LuaFunction| {
        let ctx = egui::Context::default();
        let mut ui = egui::Ui::new(
            ctx.clone(),
            egui::Id::new("temp"),
            egui::UiBuilder::default(),
        );
        
        let lua_ui = LuaUi(&mut ui as *mut egui::Ui);
        let lua_ui_userdata = lua.create_userdata(lua_ui)?;
        
        callback.call::<_, ()>(lua_ui_userdata)?;
        Ok(())
    })?)?;

    Ok(exports)
} 