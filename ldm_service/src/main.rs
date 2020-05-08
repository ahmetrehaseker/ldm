#[macro_use]
extern crate log;
extern crate ldm_commons;
extern crate ldm_metrics;
extern crate ldm_notifications;
extern crate simplelog;

use signal_hook::{iterator::Signals, SIGTERM};

use ldm_commons::AlarmSenderCommands;
use ldm_metrics::collector::{IncomingMessage, MetricCollector};
use ldm_notifications::sender::AlarmSender;
use ldm_service::parser::{get_config, LogConfiguration};
use simplelog::CombinedLogger;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn init_logger(conf: Vec<LogConfiguration>) {
    CombinedLogger::init(
        conf.iter()
            .map(|c| c.get_logger().expect("Logger must be valid"))
            .collect(),
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    // init_logger();
    let config = match get_config("ldm/config.toml") {
        Ok(config) => config,
        Err(err) => {
            panic!("Error occurred while gating config.toml, {}", err);
        }
    };
    init_logger(config.logs);
    let (notification_tx, notification_rx): (
        Sender<AlarmSenderCommands>,
        Receiver<AlarmSenderCommands>,
    ) = mpsc::channel();
    let mut metric_collector = MetricCollector::new(notification_tx.clone(), config.alarms);
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
                }
            }
        }
        Err(err) => {
            error!("Error occurred while listening signals, closing : {}", err);
            tx.send(IncomingMessage::Stop);
            notification_tx.send(AlarmSenderCommands::Stop);
        }
    }
}
