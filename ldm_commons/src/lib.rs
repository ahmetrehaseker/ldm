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

#[derive(Debug, Clone)]
pub struct Metric {}

impl Metric {
    pub fn new() -> Metric {
        Metric {}
    }
}

#[derive(Debug, Clone)]
pub enum MetricConsumerCommands {
    Send(Metric),
    Stop,
}
