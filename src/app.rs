use crate::actions::ActionsBundle;
use crate::message::MessageLog;
use crate::{binding::*, config::*, i18n::*, ui::*};
use eframe::egui;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Initializing,
    Menu,
    // SubMenu(i8),
    InGame,
    // InGamePause,
    // Invalid,
}

pub struct MyApp {
    pub state: AppState,
    pub config: Config,
    pub visuals: Vec<egui::Visuals>,
    pub selected_visual: usize,
    pub show_sidebar: bool,
    pub lua_state: Lua,
    pub locales: Rc<RefCell<LocaleTexts>>,
    pub messages: Rc<RefCell<MessageLog>>,
    pub actions_bundle: Rc<RefCell<ActionsBundle>>,
}

impl MyApp {
    pub fn new(config: Config) -> Self {
        let lua_state = Lua::new();
        let locales = LocaleTexts::new(config.env.lang.clone());
        let messages = MessageLog::default();
        let actions_bundle = ActionsBundle::new();

        Self {
            state: AppState::Initializing,
            config,
            visuals: vec![egui::Visuals::light(), egui::Visuals::dark()],
            selected_visual: 0,
            show_sidebar: true,
            lua_state,
            locales: Rc::new(RefCell::new(locales)),
            messages: Rc::new(RefCell::new(messages)),
            actions_bundle: Rc::new(RefCell::new(actions_bundle)),
        }
    }

    pub fn init_environment(&mut self) -> Result<(), mlua::Error> {
        // setup lua environment
        let lua = &self.lua_state;

        setup_lua_environment(self, lua)?;

        lua.load(self.config.lua.preload.as_str()).exec()?;

        // locales
        self.locales
            .borrow_mut()
            .load_file("app.ftl")
            .expect("Failed to load localization.");

        // run lua initialize
        lua.load(self.config.lua.initialize.as_str()).exec()?;

        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.state == AppState::Initializing {
            let result = self.init_environment();
            match result {
                Ok(_) => {}
                Err(e) => eprintln!("Error while initialization: {:?}", e),
            }

            self.messages.borrow_mut().push(
                self.locales.borrow().translate("app-info-about", None),
                1,
                1,
            );

            self.state = AppState::Menu;
            return;
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
