#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
}

impl Notification {
    pub fn new(message: String) -> Notification {
        Notification { message }
    }
}

#[derive(Debug, Clone)]
pub enum AlarmSenderCommands {
    Send(Notification),
    Stop,
}
