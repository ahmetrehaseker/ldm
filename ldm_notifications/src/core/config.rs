use crate::opsgenie::config::OpsgenieConfiguration;
use crate::slack::config::SlackConfiguration;
use ldm_commons::Notification;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum NotificationConfiguration {
    #[serde(rename = "opsgenie")]
    OpsGenie(OpsgenieConfiguration),
    #[serde(rename = "slack")]
    Slack(SlackConfiguration),
}

pub trait Sender: Debug + Send + Sync {
    fn send(&self, notification: &Notification);
}
