use crate::alert::models::{AlertCreateResponse, AlertData, AlertStatus, AlertStatusResponse};
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    request_id: String,
    key: String,
    alert_status: Option<AlertStatus>,
}
impl Alert {
    pub fn create(key: &String, data: AlertData) -> Result<Alert, reqwest::Error> {
        let request_url = "https://api.opsgenie.com/v2/alerts";
        let mut response = Client::new()
            .post(request_url)
            .header(AUTHORIZATION, format!("GenieKey {}", key))
            .json(&data)
            .send()?;
        let resp: AlertCreateResponse<String> = response.json()?;
        Ok(Alert {
            request_id: resp.request_id,
            key: key.clone(),
            alert_status: None,
        })
    }
    pub fn status(mut self) -> Result<Option<AlertStatus>, reqwest::Error> {
        let request_url = format!(
            "https://api.opsgenie.com/v2/alerts/requests/{}",
            self.request_id
        );
        let mut response = Client::new()
            .get(&request_url)
            .header(AUTHORIZATION, format!("GenieKey {}", self.key))
            .send()?;
        let resp: AlertStatusResponse<AlertStatus> = response.json()?;
        self.alert_status = Some(resp.data);
        Ok(self.alert_status)
    }
    pub fn close(&mut self) -> Result<(), reqwest::Error> {
        let request_url = format!(
            "https://api.opsgenie.com/v2/alerts/requests/{}",
            self.request_id
        );
        let mut response = Client::new()
            .get(&request_url)
            .header(AUTHORIZATION, format!("GenieKey {}", self.key))
            .send()?;
        let resp: AlertStatusResponse<AlertStatus> = response.json()?;
        let request_url = format!(
            "https://api.opsgenie.com/v2/alerts/{}/close",
            resp.data.alert_id
        );
        Client::new()
            .post(&request_url)
            .header(AUTHORIZATION, format!("GenieKey {}", self.key))
            .json(&json!({}))
            .send()?;

        Ok(())
    }
}
