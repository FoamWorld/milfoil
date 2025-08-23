use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use mlua::{FromLua, Value};

#[derive(Clone)]
pub struct Message {
    text: String,
    mode: u8,
    pub level: i32,
}

impl Message {
    pub fn display(&self, ui: &mut egui::Ui) {
        if self.mode == 1 {
            let mut cache = CommonMarkCache::default();
            CommonMarkViewer::new().show(ui, &mut cache, self.text.as_str());
        } else {
            ui.label(self.text.as_str());
        }
    }
}

#[derive(Default, Clone)]
pub struct MessageLog {
    pub messages: Vec<Message>,
}

impl MessageLog {
    pub fn push(&mut self, text: String, mode: u8, level: i32) {
        self.messages.push(Message { text, mode, level });
    }

    pub fn append(&mut self, mut other: MessageLog) {
        self.messages.append(&mut other.messages);
    }

    pub fn clear(&mut self, limit: i32, decrease: i32) {
        self.messages.retain_mut(|message| {
            if message.level < limit {
                return false;
            } else {
                message.level -= decrease;
                return true;
            }
        });
    }
}

impl FromLua for MessageLog {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let mut log = MessageLog::default();
        if let Value::Table(table) = value {
            let len = table.len()?;
            for i in 1..=len {
                let msg: mlua::Table = table.raw_get(i)?;
                let text = msg.raw_get("text")?;
                let mode = msg.raw_get("mode")?;
                let level = msg.raw_get("level")?;
                log.push(text, mode, level);
            }
        }
        Ok(log)
    }
}
