#[macro_use]
extern crate log;
extern crate ldm_commons;
extern crate ldm_metrics;
extern crate ldm_notifications;

use signal_hook::{iterator::Signals, SIGTERM};

use ldm_commons::{AlarmSenderCommands, MetricConsumerCommands};
use ldm_metrics::collector::{IncomingMessage, MetricCollector};
use ldm_notifications::sender::AlarmSender;
use ldm_service::parser::{get_config, get_config_dir};
use metric_consumer::consumer::MetricConsumer;
use std::io::Error;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[tokio::main]
async fn main() {
    match get_config_dir() {
        Ok(mut path) => {
            path.push("log4rs.yaml");
            log4rs::init_file(path, Default::default()).unwrap();
        }
        Err(err) => panic!("Error while reading log conf: {}", err),
    }

    let config = match get_config("ldm/config.toml") {
        Ok(config) => config,
        Err(err) => {
            panic!("Error occurred while gating config.toml, {}", err);
        }
    };
    let (notification_tx, notification_rx): (
        Sender<AlarmSenderCommands>,
        Receiver<AlarmSenderCommands>,
    ) = mpsc::channel();
    let (metric_tx, metric_rx): (
        Sender<MetricConsumerCommands>,
        Receiver<MetricConsumerCommands>,
    ) = mpsc::channel();
    let mut metric_consumer = MetricConsumer::new(metric_rx, config.consumers);
    std::thread::spawn(move || metric_consumer.start());
    let mut metric_collector =
        MetricCollector::new(notification_tx.clone(), metric_tx.clone(), config.metrics);
    let (tx, rx): (Sender<IncomingMessage>, Receiver<IncomingMessage>) = mpsc::channel();
    std::thread::spawn(move || metric_collector.start(rx));
    let mut alarm_sender = AlarmSender::new(notification_rx, config.notifications);
    std::thread::spawn(move || alarm_sender.start());
    info!("Setup complete for {}", config.device);
    match Signals::new(&[SIGTERM]) {
        Ok(signals) => {
            for sig in signals.wait() {
                if sig == SIGTERM {
                    tx.send(IncomingMessage::Stop);
                    notification_tx.send(AlarmSenderCommands::Stop);
                    metric_tx.send(MetricConsumerCommands::Stop);
                }
            }
        }
        Err(err) => {
            error!("Error occurred while listening signals, closing : {}", err);
            tx.send(IncomingMessage::Stop);
            notification_tx.send(AlarmSenderCommands::Stop);
            metric_tx.send(MetricConsumerCommands::Stop);
        }
    }
}
