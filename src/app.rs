#[derive(Debug, Clone)]
pub struct TwitchMessage {
    pub user: String,
    pub message: String,
}
pub enum AppMode {
    Normal,
    ChatEnter,
}

pub struct App {
    pub chat_input: String,
    pub app_mode: AppMode,
    pub messages: Vec<TwitchMessage>,
}

impl App {
    pub fn new() -> App {
        App {
            chat_input: String::new(),
            app_mode: AppMode::Normal,
            messages: Vec::new(),
        }
    }

    pub fn send_message(&mut self) {}

    pub fn receieve_msg(&mut self, message: &TwitchMessage) {
        self.messages.insert(0, message.clone());
    }
}
