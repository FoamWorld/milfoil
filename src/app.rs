use crate::message::MessageLog;
use crate::{config::*, ui::*};
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
    pub config: Config,
    pub visuals: Vec<egui::Visuals>,
    pub selected_visual: usize,
    pub show_sidebar: bool,
    pub lua_state: Lua,
    pub messages: Rc<RefCell<MessageLog>>,
}

impl MyApp {
    pub fn new(config: Config) -> Self {
        let lua = Lua::new();

        let messages = Rc::new(RefCell::new(MessageLog::default()));

        Self {
            state: AppState::Loading,
            config,
            visuals: vec![egui::Visuals::light(), egui::Visuals::dark()],
            selected_visual: 0,
            show_sidebar: true,
            lua_state: lua,
            messages,
        }
    }

    pub fn init_environment(&self) -> Result<(), mlua::Error> {
        let lua = &self.lua_state;

        let app_module = lua.create_table()?;
        app_module.set("version", "0.1")?;

        let queue_module = lua.create_table()?;
        queue_module.set("MODE_PLAIN", 0)?;
        queue_module.set("MODE_MARKDOWN", 1)?;
        {
            let ref_messages = Rc::clone(&self.messages);
            queue_module.set(
                "push_message",
                lua.create_function(move |_, (text, mode, level): (String, u8, i32)| {
                    ref_messages.borrow_mut().push(text, mode, level);
                    Ok(())
                })?,
            )?;
        }
        {
            let ref_messages = Rc::clone(&self.messages);
            queue_module.set(
                "clear",
                lua.create_function(move |_, (limit, decrease): (i32, i32)| {
                    ref_messages.borrow_mut().clear(limit, decrease);
                    Ok(())
                })?,
            )?;
        }

        app_module.set("queue", queue_module)?;
        lua.globals()
            .get::<mlua::Table>("package")?
            .get::<mlua::Table>("loaded")?
            .set("app", app_module)?;

        lua.load(self.config.lua.preload.as_str()).exec()?;
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.state == AppState::Loading {
            let result = self.init_environment();
            match result {
                Ok(_) => {}
                Err(e) => eprintln!("Error while initialization: {:?}", e),
            }

            self.state = AppState::Running;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_sidebar = !self.show_sidebar;
        }

        setup_custom_styles(ctx);

        if self.show_sidebar {
            add_sidebar(self, ctx);
        }

        add_central(self, ctx);
    }
}
