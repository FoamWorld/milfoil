#[derive(Default, Clone)]
pub struct MessageLog {
    pub messages: Vec<String>,
}

impl MessageLog {
    pub fn add_message(&mut self, text: String) {
        self.messages.push(text);
    }
}
