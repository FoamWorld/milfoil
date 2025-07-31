use crate::{actions::RefActionsBundle, app::MyApp};
use mlua::{Error, Function, IntoLua, Lua, Table};
use std::rc::Rc;

pub fn setup_lua_environment(app: &MyApp, lua: &Lua) -> Result<(), mlua::Error> {
    let module = lua.create_table()?;
    module.set("version", "0.1")?;
    module.set("update", lua.create_function(|_, _: ()| Ok(()))?)?;
    module.set("quit", lua.create_function(|_, _: ()| Ok(()))?)?;
    module.set("actions", setup_actions_interface(app))?;
    module.set("i18n", setup_i18n_module(app, lua)?)?;
    module.set("queue", setup_queue_module(app, lua)?)?;
    lua.globals()
        .get::<Table>("package")?
        .get::<Table>("loaded")?
        .set("app", module)?;
    Ok(())
}

pub fn run_registered(lua: &Lua, key: &str) -> Result<(), mlua::Error> {
    let update = lua
        .globals()
        .get::<Table>("package")?
        .get::<Table>("loaded")?
        .get::<Table>("app")?
        .get::<Function>(key)?;
    update.call(())
}

pub fn setup_actions_interface(app: &MyApp) -> impl IntoLua {
    let actions = Rc::clone(&app.actions_bundle);
    RefActionsBundle(actions)
}

pub fn setup_i18n_module(app: &MyApp, lua: &Lua) -> Result<Table, Error> {
    let i18n_module = lua.create_table()?;
    {
        let lc = Rc::clone(&app.locales);
        i18n_module.set(
            "load_file",
            lua.create_function(move |_, path: String| {
                lc.borrow_mut().load_file(path.as_str());
                Ok(())
            })?,
        )?;
    }
    {
        let lc = Rc::clone(&app.locales);
        i18n_module.set(
            "t",
            lua.create_function(move |_, key: String| {
                let translation = lc.borrow().translate(key.as_str(), None);
                Ok(translation)
            })?,
        )?;
    }
    {
        let lc = Rc::clone(&app.locales);
        i18n_module.set(
            "_",
            lua.create_function(move |_, key: String| {
                if let Some(translation) = lc.borrow().try_translate(key.as_str(), None) {
                    Ok(translation)
                } else {
                    Ok("".to_string())
                }
            })?,
        )?;
    }
    Ok(i18n_module)
}

pub fn setup_queue_module(app: &MyApp, lua: &Lua) -> Result<Table, Error> {
    let queue_module = lua.create_table()?;
    queue_module.set("MODE_PLAIN", 0)?;
    queue_module.set("MODE_MARKDOWN", 1)?;
    {
        let ref_messages = Rc::clone(&app.messages);
        queue_module.set(
            "push_message",
            lua.create_function(move |_, (text, mode, level): (String, u8, i32)| {
                ref_messages.borrow_mut().push(text, mode, level);
                Ok(())
            })?,
        )?;
    }
    {
        let ref_messages = Rc::clone(&app.messages);
        queue_module.set(
            "clear",
            lua.create_function(move |_, (limit, decrease): (i32, i32)| {
                ref_messages.borrow_mut().clear(limit, decrease);
                Ok(())
            })?,
        )?;
    }
    Ok(queue_module)
}
