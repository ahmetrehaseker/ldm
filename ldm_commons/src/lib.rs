#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub priority: String,
    pub description: String,
}

impl Notification {
    pub fn new(message: String, priority: String, description: String) -> Notification {
        Notification {
            message,
            priority,
            description,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AlarmSenderCommands {
    Send(Notification),
    Stop,
}

#[derive(Debug, Clone)]
pub struct MetricData {
    name: String,
    value: f64,
}

impl MetricData {
    pub fn new(name: String, value: f64) -> MetricData {
        MetricData { name, value }
    }
}

#[derive(Debug, Clone)]
pub enum MetricConsumerCommands {
    Send(MetricData),
    Stop,
}
