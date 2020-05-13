use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertCreateResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub took: f32,
    pub request_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertStatusResponse<T> {
    pub data: T,
    pub took: f32,
    pub request_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertStatus {
    pub success: bool,
    pub action: String,
    pub processed_at: String,
    pub integration_id: String,
    pub is_success: bool,
    pub status: String,
    pub alert_id: String,
    pub alias: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
    P5,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responder {
    id: String,
    r#type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertData {
    /// Message of the alert
    pub message: String,
    /// Client-defined identifier of the alert, that is also the key element of Alert De-Duplication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Description field of the alert that is generally used to provide a detailed information about the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Teams, users, escalations and schedules that the alert will be routed to send notifications.
    /// type field is mandatory for each item, where possible values are team, user, escalation and schedule.
    /// If the API Key belongs to a team integration, this field will be overwritten with the owner team.
    /// Either id or name of each responder should be provided.You can refer below for example values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responders: Option<Vec<Responder>>,
    /// Teams and users that the alert will become visible to without sending any notification.type
    /// field is mandatory for each item, where possible values are team and user.
    /// In addition to the type field, either id or name should be given for teams and either
    /// id or username should be given for users. Please note: that alert will be visible to
    /// the teams that are specified withinresponders field by default, so there is no need
    /// to re-specify them within visibleTo field. You can refer below for example values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to: Option<Vec<Responder>>,
    /// Custom actions that will be available for the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Tags of the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Map of key-value pairs to use as custom properties of the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>, // TODO: that should be key value pair.(Use HashMap)
    /// Entity field of the alert that is generally used to specify which domain alert is related to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    /// Source field of the alert. Default value is IP address of the incoming request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Priority level of the alert. Possible values are P1, P2, P3, P4 and P5. Default value is P3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    /// Display name of the request owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Additional note that will be added while creating the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
impl AlertData {
    pub fn new(message: String) -> AlertData {
        AlertData {
            message: message.clone(),
            alias: None,
            description: None,
            responders: None,
            visible_to: None,
            actions: None,
            tags: None,
            details: None,
            entity: None,
            source: None,
            priority: None,
            user: None,
            note: None,
        }
    }
    pub fn alias(mut self, alias: String) -> AlertData {
        self.alias = Some(alias.clone());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> AlertData {
        self.tags = Some(tags.clone());
        self
    }
    pub fn entity(mut self, entity: String) -> AlertData {
        self.entity = Some(entity);
        self
    }
    pub fn source(mut self, source: String) -> AlertData {
        self.source = Some(source.clone());
        self
    }
    pub fn priority(mut self, priority: Priority) -> AlertData {
        self.priority = Some(priority);
        self
    }
}
