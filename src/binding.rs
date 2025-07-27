use crate::app::MyApp;
use mlua::{Error, Lua, Table};
use std::rc::Rc;

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
