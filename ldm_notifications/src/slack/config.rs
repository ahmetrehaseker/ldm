extern crate slack;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SlackConfiguration {
    pub key: String,
    pub channel: String,
}
