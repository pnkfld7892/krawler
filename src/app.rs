#[derive(Debug, Clone)]
pub struct TwitchMessage {
    pub user: String,
    pub message: String,
}
pub enum AppMode {
    ScrollChat,
    ChatEnter,
}

pub struct App {
    pub chat_input: String,
    pub app_mode: Option<AppMode>,
    pub messages: Vec<TwitchMessage>,
}

impl App {
    pub fn new() -> App {
        App {
            chat_input: String::new(),
            app_mode: None,
            messages: Vec::new(),
        }
    }

    pub fn send_message(&mut self) {}

    pub fn receieve_msg(&mut self, message: &TwitchMessage) {
        self.messages.push(message.clone());
    }
}
