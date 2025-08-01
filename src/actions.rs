use mlua::{FromLuaMulti, Function, Lua, Table, UserData, Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Action {
    pub name: String,
    pub func: Function,
}

pub struct Actions {
    pub name: String,
    pub list: Vec<Action>,
}

impl Actions {
    fn new(name: String) -> Self {
        Self {
            name,
            list: Vec::<Action>::new(),
        }
    }
}

impl Default for Actions {
    fn default() -> Self {
        Actions::new("???".to_string())
    }
}

impl FromLuaMulti for Actions {
    fn from_lua_multi(mut values: mlua::MultiValue, _: &Lua) -> mlua::Result<Self> {
        let name = if let Some(first) = values.pop_front() {
            if let Value::String(string) = first {
                string.to_string_lossy()
            } else {
                return Ok(Actions::default());
            }
        } else {
            return Ok(Actions::default());
        };
        let mut list = Vec::<Action>::new();
        if let Some(second) = values.pop_front() {
            if let Value::Table(table) = second {
                let len = table.len()?;
                for i in 1..=len {
                    let tab: Table = table.get(i)?;
                    let a_name: String = tab.get("name")?;
                    let a_func: Function = tab.get("func")?;
                    list.push(Action {
                        name: a_name,
                        func: a_func,
                    });
                }
            } else {
                return Ok(Actions::new(name));
            }
        } else {
            return Ok(Actions::new(name));
        };
        Ok(Actions { name, list })
    }
}

pub struct ActionsBundle {
    pub dict: HashMap<String, Actions>,
}

impl ActionsBundle {
    pub fn new() -> Self {
        Self {
            dict: HashMap::<_, _>::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dict.is_empty()
    }
}

pub struct RefActionsBundle(pub Rc<RefCell<ActionsBundle>>);

impl UserData for RefActionsBundle {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("clear", |_, this, _: ()| {
            let mut bundle = this.0.borrow_mut();
            bundle.dict.clear();
            Ok(())
        });

        methods.add_method("remove", |_, this, key: String| {
            let mut bundle = this.0.borrow_mut();
            bundle.dict.remove(&key);
            Ok(())
        });

        methods.add_method("insert", |_, this, (key, value): (String, Actions)| {
            let mut bundle = this.0.borrow_mut();
            bundle.dict.insert(key, value);
            Ok(())
        });
    }
}
