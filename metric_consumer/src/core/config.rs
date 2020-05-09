use crate::file::config::FileConsumerConfiguration;
use ldm_commons::Metric;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum MetricConsumerConfiguration {
    #[serde(rename = "file")]
    File(FileConsumerConfiguration),
}

pub trait Consumer: Debug + Send + Sync {
    fn consume(&self, metric: Metric);
}
