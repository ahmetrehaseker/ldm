use crate::core::config::Sender;
use ldm_commons::Notification;
use opsgenie_rs::alert::models::{AlertData, Priority};
use opsgenie_rs::alert::operations::Alert;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpsgenieConfiguration {
    pub key: String,
}

#[derive(Debug)]
pub struct OpsGenieSender {
    pub key: String,
}

impl Sender for OpsGenieSender {
    fn send(&self, notification: &Notification) {
        let data = AlertData {
            message: notification.message.clone(),
            alias: Some(notification.message.clone()),
            description: Some(notification.description.clone()),
            responders: None,
            visible_to: None,
            actions: None,
            tags: Some(vec![String::from("ldm")]),
            details: None,
            entity: None,
            source: None,
            priority: Some(if notification.priority == "high" {
                Priority::P2
            } else {
                Priority::P4
            }),
            user: None,
            note: None,
        };
        let result = Alert::create(&self.key, data);
        match result {
            Ok(_) => info!("Alert created successfully"),
            Err(err) => error!("Error occurred while creating alert {}", err),
        }
    }
}
