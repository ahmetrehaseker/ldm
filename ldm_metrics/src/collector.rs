use crate::core::config::{Alarm, AlarmConfiguration, AlarmStatus};
use crate::cpu::metric::CpuUsedAlarm;
use crate::disk::metric::DiskUsedAlarm;
use crate::temp::metric::TemperatureAlarm;
use clokwerk::{Interval, Scheduler};
use ldm_commons::{AlarmSenderCommands, Notification};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use crate::errors;
use crate::mem::metric::MemUsageAlarm;
use crate::network::metric::{NetworkInAlarm, NetworkOutAlarm};
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
    configurations: Vec<AlarmConfiguration>,
}

impl MetricCollector {
    pub fn new(
        notification_channel: Sender<AlarmSenderCommands>,
        configurations: Vec<AlarmConfiguration>,
    ) -> MetricCollector {
        MetricCollector {
            notification_channel,
            configurations,
        }
    }

    pub fn start(&mut self, rx: Receiver<IncomingMessage>) {
        let alarms = self.setup_metrics();
        let mut scheduler = Scheduler::new();
        for mut alarm in alarms {
            info!("{:?}", alarm);
            let sender = self.notification_channel.clone();
            scheduler
                .every(Interval::Seconds(alarm.get_period()))
                .run(move || collect(alarm.borrow_mut(), &sender));
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

    fn setup_metrics(&self) -> Vec<Box<dyn Alarm>> {
        let mut metrics: Vec<Box<dyn Alarm>> = Vec::new();
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

    pub fn setup_metric(
        &self,
        configuration: &AlarmConfiguration,
    ) -> Result<Box<dyn Alarm>, Error> {
        match configuration.kind() {
            "cpu::used" => Ok(Box::new(CpuUsedAlarm::new(configuration.clone()))),
            "mem::usage" => Ok(Box::new(MemUsageAlarm::new(configuration.clone()))),
            "disk::used" => Ok(Box::new(DiskUsedAlarm::new(configuration.clone()))),
            "network::in" => Ok(Box::new(NetworkInAlarm::new(configuration.clone()))),
            "network::out" => Ok(Box::new(NetworkOutAlarm::new(configuration.clone()))),
            "temp" => Ok(Box::new(TemperatureAlarm::new(configuration.clone()))),
            _ => Err(Error::Generic(format!(
                "Metric {} is not supported",
                configuration.kind(),
            ))),
        }
    }
}

fn collect(alarm: &mut Box<dyn Alarm>, tx: &Sender<AlarmSenderCommands>) {
    if let Err(e) = alarm.poll_metric() {
        error!("{}", e)
    }
    match alarm.check_conditions() {
        Ok(result) => {
            match result {
                AlarmStatus::Ok => match alarm.previous_status() {
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
                AlarmStatus::Alarm => match alarm.previous_status() {
                    AlarmStatus::Ok | AlarmStatus::NoData => {
                        if let Err(err) = tx.send(AlarmSenderCommands::Send(Notification::new(
                            alarm.get_message(),
                        ))) {
                            error!("Error while sending to channel {}", err);
                        } else {
                            info!("State changed to Alarm");
                        }
                    }
                    AlarmStatus::Alarm => {
                        debug!("No change");
                    }
                },
                AlarmStatus::NoData => {}
            }
            alarm.set_status(result);
        }
        Err(err) => error!("Error occurred: {}", err),
    }
}

pub enum MetricTypes {
    CpuUsed(CpuUsedAlarm),

    // MemUsed(MemUsageAlarm),
    // MemTotal(MemTotalAlarm),
    // MemFree(MemFreeAlarm),
    //
    DiskUsed(DiskUsedAlarm),
    //
    // NetworkIn(NetworkInAlarm),
    // NetworkOut(NetworkOutAlarm),
    // NetworkTotal(NetworkTotalAlarm),
    //
    Temp(TemperatureAlarm),
}
