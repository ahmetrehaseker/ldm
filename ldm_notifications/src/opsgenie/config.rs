use crate::core::config::Sender;
use ldm_commons::Notification;
use opsgenie_rs::alert::models::AlertData;
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
        let data = AlertData::new(notification.message.clone());
        let result = Alert::create(&self.key, data);
        match result {
            Ok(_) => info!("Alert created successfully"),
            Err(err) => error!("Error occurred while creating alert {}", err),
        }
    }
}
