use crate::core::config::{Consumer, MetricConsumerConfiguration};
use crate::file::config::FileConsumer;
use ldm_commons::MetricConsumerCommands;
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct MetricConsumer {
    metric_channel: Receiver<MetricConsumerCommands>,
    configuration: Vec<MetricConsumerConfiguration>,
}

impl MetricConsumer {
    pub fn new(
        metric_channel: Receiver<MetricConsumerCommands>,
        configurations: Vec<MetricConsumerConfiguration>,
    ) -> MetricConsumer {
        MetricConsumer {
            metric_channel,
            configuration: configurations,
        }
    }

    pub fn start(&mut self) {
        let consumers = self.setup_consumers();
        loop {
            match self.metric_channel.recv() {
                Ok(command) => match command {
                    MetricConsumerCommands::Send(metric) => {
                        for consumer in &consumers {
                            consumer.consume(metric.clone());
                        }
                    }
                    MetricConsumerCommands::Stop => {
                        info!("Stopping Consumer");
                        break;
                    }
                },
                Err(err) => {
                    error!("{}", err);
                    break;
                }
            }
        }
    }

    fn setup_consumers(&mut self) -> Vec<Box<dyn Consumer>> {
        let mut consumers: Vec<Box<dyn Consumer>> = Vec::new();
        for conf in &self.configuration {
            match conf {
                MetricConsumerConfiguration::File(f) => {
                    consumers.push(Box::new(FileConsumer::new()))
                }
            }
        }
        consumers
    }
}
