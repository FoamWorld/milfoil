use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct MessageLog {
    pub messages: Arc<Mutex<Vec<String>>>,
}

impl MessageLog {
    pub fn add_message(&self, text: String) {
        self.messages.lock().unwrap().push(text);
    }
}
