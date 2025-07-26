use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

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
