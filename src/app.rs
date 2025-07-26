use crate::message::MessageLog;
use crate::{config, ui::*};
use eframe::egui;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Loading,
    Running,
}

pub struct MyApp {
    pub state: AppState,
    pub show_sidebar: bool,
    pub lua_state: Lua,
    pub message_log: Rc<RefCell<MessageLog>>,
}

impl MyApp {
    pub fn new() -> Self {
        let lua = Lua::new();

        let message_log = Rc::new(RefCell::new(MessageLog::default()));

        Self {
            state: AppState::Loading,
            show_sidebar: true,
            lua_state: lua,
            message_log,
        }
    }

    pub fn init_environment(&self) -> Result<(), mlua::Error> {
        let lua = &self.lua_state;
        let p = Rc::clone(&self.message_log);
        let message_module = lua.create_table()?;
        message_module.set("version", "0.1")?;
        message_module.set(
            "push",
            lua.create_function(move |_, text: String| {
                p.borrow_mut().add_message(text);
                Ok(())
            })?,
        )?;
        lua.globals()
            .get::<mlua::Table>("package")?
            .get::<mlua::Table>("loaded")?
            .set("app", message_module)?;

        lua.load(config::LUA_PRELOAD).exec()?;
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.state == AppState::Loading {
            let result = self.init_environment();
            match result {
                Ok(_) => {}
                Err(e) => println!("Error while initialization: {:?}", e),
            }

            self.state = AppState::Running;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_sidebar = !self.show_sidebar;
        }

        if self.show_sidebar {
            add_sidebar(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.style_mut(|style| {
                style.spacing.item_spacing = egui::Vec2::new(12.0, 8.0);
            });
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for msg in self.message_log.borrow().messages.iter() {
                        ui.horizontal(|ui| {
                            ui.label(msg);
                        });
                    }

                    if ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                    }
                });
        });
    }
}
