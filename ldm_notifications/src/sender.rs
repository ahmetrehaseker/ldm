use crate::core::config::{NotificationConfiguration, Sender};
use crate::opsgenie::config::OpsGenieSender;
use ldm_commons::AlarmSenderCommands;
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct AlarmSender {
    notification_channel: Receiver<AlarmSenderCommands>,
    configurations: Vec<NotificationConfiguration>,
}

impl AlarmSender {
    pub fn new(
        notification_channel: Receiver<AlarmSenderCommands>,
        configurations: Vec<NotificationConfiguration>,
    ) -> AlarmSender {
        AlarmSender {
            notification_channel,
            configurations,
        }
    }

    pub fn start(&mut self) {
        let senders = self.setup_senders();
        loop {
            match self.notification_channel.recv() {
                Ok(command) => match command {
                    AlarmSenderCommands::Stop => {
                        info!("Stopping Sender");
                        break;
                    }
                    AlarmSenderCommands::Send(notification) => {
                        info!("Send Message received");
                        for sender in &senders {
                            sender.send(&notification);
                        }
                    }
                },
                Err(err) => {
                    error!("{}", err);
                    break;
                }
            }
        }
    }

    fn setup_senders(&mut self) -> Vec<Box<dyn Sender>> {
        let mut senders: Vec<Box<dyn Sender>> = Vec::new();
        for conf in &self.configurations {
            match conf {
                NotificationConfiguration::OpsGenie(conf) => {
                    senders.push(Box::new(OpsGenieSender {
                        key: String::from(&conf.key),
                    }))
                }
                NotificationConfiguration::Slack(_) => {}
            }
        }
        senders
    }
}
