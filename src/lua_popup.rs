use mlua::prelude::*;
use eframe::egui;
use std::sync::mpsc;
use crate::popup::{PopupMessage};
pub struct LuaPopup {
    ctx: egui::Context,
    error_sender: mpsc::Sender<String>,
    popup_message: PopupMessage,
}
impl LuaUserData for LuaPopup {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("show_error", |_lua, this, message: String| {
            this.popup_message.show_error(message);
            Ok(())
        });
        methods.add_method("show_warning", |_lua, this, message: String| {
            this.popup_message.show_warning(message);
            Ok(())
        });
        methods.add_method("show_info", |_lua, this, message: String| {
            this.popup_message.show_info(message);
            Ok(())
        });
        methods.add_method("show_panic", |_lua, this, message: String| {
            this.popup_message.show_panic(message);
            Ok(())
        });
    }
}
pub fn create_lua_popup_module(lua: &Lua, ctx: egui::Context, error_sender: mpsc::Sender<String>) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let lua_popup = LuaPopup {
        ctx: ctx.clone(),
        error_sender: error_sender.clone(),
        popup_message: PopupMessage::default(),
    };
    let lua_popup_userdata = lua.create_userdata(lua_popup)?;
    exports.set("popup", lua_popup_userdata)?;
    Ok(exports)
} 