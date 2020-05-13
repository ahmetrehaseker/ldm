use crate::core::config::{AlarmStatus, Metric, MetricConfiguration};
use crate::cpu::metric::CpuUsageMetric;
use crate::disk::metric::DiskUsageMetric;
use crate::temp::metric::TemperatureMetric;
use clokwerk::{Interval, Scheduler};
use ldm_commons::{AlarmSenderCommands, MetricConsumerCommands, MetricData, Notification};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use crate::errors;
use crate::mem::metric::MemoryUsageMetric;
use crate::network::metric::{
    NetworkRxTotalMetric, NetworkRxUsageMetric, NetworkTxTotalMetric, NetworkTxUsageMetric,
};
use errors::*;
use std::borrow::BorrowMut;

#[derive(Debug)]
pub enum IncomingMessage {
    Start,
    Stop,
}

#[derive(Debug)]
pub struct MetricCollector {
    notification_channel: Sender<AlarmSenderCommands>,
    metric_channel: Sender<MetricConsumerCommands>,
    configurations: Vec<MetricConfiguration>,
}

impl MetricCollector {
    pub fn new(
        notification_channel: Sender<AlarmSenderCommands>,
        metric_channel: Sender<MetricConsumerCommands>,
        configurations: Vec<MetricConfiguration>,
    ) -> MetricCollector {
        MetricCollector {
            notification_channel,
            metric_channel,
            configurations,
        }
    }

    pub fn start(&mut self, rx: Receiver<IncomingMessage>) {
        let metrics = self.setup_metrics();
        let mut scheduler = Scheduler::new();
        for mut metric in metrics {
            info!("{:?}", metric);
            let sender = self.notification_channel.clone();
            let metric_channel = self.metric_channel.clone();
            scheduler
                .every(Interval::Seconds(metric.get_period()))
                .run(move || collect(metric.borrow_mut(), &sender, &metric_channel));
        }
        let handle = scheduler.watch_thread(Duration::from_secs(5));
        loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    IncomingMessage::Start => info!("Collector Already started"),
                    IncomingMessage::Stop => {
                        info!("Stopping Collector");
                        handle.stop();
                        break;
                    }
                },
                Err(err) => {
                    error!("{}", err);
                    handle.stop();
                    break;
                }
            }
        }
    }

    fn setup_metrics(&self) -> Vec<Box<dyn Metric>> {
        let mut metrics: Vec<Box<dyn Metric>> = Vec::new();
        for conf in &self.configurations {
            match self.setup_metric(conf) {
                Ok(alarm) => {
                    metrics.push(alarm);
                }
                Err(err) => error!("{}", err),
            }
        }
        return metrics;
    }

    fn setup_metric(&self, configuration: &MetricConfiguration) -> Result<Box<dyn Metric>, Error> {
        match configuration.name.as_str() {
            "cpu::usage" => Ok(Box::new(CpuUsageMetric::new(configuration.clone()))),
            "memory::usage" => Ok(Box::new(MemoryUsageMetric::new(configuration.clone()))),
            "disk::usage" => Ok(Box::new(DiskUsageMetric::new(configuration.clone()))),
            "network::rx::usage" => Ok(Box::new(NetworkRxUsageMetric::new(configuration.clone()))),
            "network::rx::total" => Ok(Box::new(NetworkRxTotalMetric::new(configuration.clone()))),
            "network::tx::usage" => Ok(Box::new(NetworkTxUsageMetric::new(configuration.clone()))),
            "network::tx::total" => Ok(Box::new(NetworkTxTotalMetric::new(configuration.clone()))),
            // "disk::read::io" => Ok(Box::new(NetworkTxTotalMetric::new(configuration.clone()))),
            // "disk::write::io" => Ok(Box::new(NetworkTxTotalMetric::new(configuration.clone()))),
            // "process::network" => Ok(Box::new(NetworkTxTotalMetric::new(configuration.clone()))),
            // "process::cpu" => Ok(Box::new(NetworkTxTotalMetric::new(configuration.clone()))),
            // "process::memory" => Ok(Box::new(NetworkTxTotalMetric::new(configuration.clone()))),
            // "socket::size" => Ok(Box::new(NetworkTxTotalMetric::new(configuration.clone()))),
            "temperature" => Ok(Box::new(TemperatureMetric::new(configuration.clone()))),
            _ => Err(Error::Generic(format!(
                "Metric {} is not supported",
                configuration.name,
            ))),
        }
    }
}

fn collect(
    metric: &mut Box<dyn Metric>,
    tx: &Sender<AlarmSenderCommands>,
    metric_tx: &Sender<MetricConsumerCommands>,
) {
    match metric.poll_metric() {
        Ok(data) => {
            metric_tx.send(MetricConsumerCommands::Send(MetricData::new(
                metric.get_name(),
                data,
            )));
            for alarm in metric.get_alarms() {
                match alarm.check(data) {
                    Err(err) => error!("Error occurred: {}", err),
                    Ok(result) => {
                        match result {
                            AlarmStatus::Ok => match alarm.previous_status {
                                AlarmStatus::Ok => {
                                    debug!("No change");
                                }
                                AlarmStatus::Alarm => {
                                    debug!("State changed from Alarm to Ok");
                                }
                                AlarmStatus::NoData => {
                                    debug!("State changed from NoData to Ok");
                                }
                            },
                            AlarmStatus::Alarm => {
                                let desc = format!(
                                    "Alarm data set -> {}",
                                    alarm
                                        .samples
                                        .iter()
                                        .map(ToString::to_string)
                                        .collect::<Vec<String>>()
                                        .join(":")
                                );
                                if let Err(err) =
                                    tx.send(AlarmSenderCommands::Send(Notification::new(
                                        alarm.config.name(),
                                        alarm.config.severity(),
                                        desc,
                                    )))
                                {
                                    error!("Error while sending to channel {}", err);
                                } else {
                                    info!("State changed to Alarm");
                                }
                            }
                            AlarmStatus::NoData => {}
                        }
                        alarm.set_status(result);
                    }
                }
            }
        }
        Err(err) => error!("{}", err),
    }
}
