// use crate::core::config::NotificationConfiguration;
// use crate::opsgenie::config::OpsGenieSender;
//
// pub struct NotificationSender {
//     senders: Vec<Box<dyn Sender>>,
// }
//
// impl NotificationSender {
//     pub fn new() -> NotificationSender {
//         NotificationSender { senders: vec![] }
//     }
//     pub fn add_sender(&mut self, sender: Box<dyn Sender>) {
//         self.senders.push(sender);
//     }
// }
//
// pub fn setup_senders(configurations: Vec<NotificationConfiguration>) -> NotificationSender {
//     let mut sender = NotificationSender::new();
//     for conf in configurations {
//         match conf {
//             NotificationConfiguration::OpsGenie(conf) => {
//                 sender.add_sender(Box::new(OpsGenieSender { key: conf.key }))
//             }
//             NotificationConfiguration::Slack(conf) => {}
//         }
//     }
//     sender
// }
