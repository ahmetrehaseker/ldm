use crate::core::config::Consumer;
use ldm_commons::Metric;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FileConsumerConfiguration {
    path: String,
    rotation: u32,
}

#[derive(Debug)]
pub struct FileConsumer {}

impl FileConsumer {
    pub fn new() -> FileConsumer {
        FileConsumer {}
    }
}

impl Consumer for FileConsumer {
    fn consume(&self, metric: Metric) {
        unimplemented!()
    }
}
