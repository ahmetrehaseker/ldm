#[macro_use]
extern crate log;
extern crate simplelog;
extern crate ldm_commons;
extern crate ldm_metrics;

use std::time::Duration;
use ldm_service::parser::{LogConfiguration, get_config};
use simplelog::CombinedLogger;
use ldm_metrics::core::config::{Alarm, AlarmStatus};
use clokwerk::{Scheduler, Interval};
use ldm_metrics::setup::setup_metrics;
use std::borrow::BorrowMut;

pub fn init_logger(conf: Vec<LogConfiguration>) {
    CombinedLogger::init(
        conf.iter().map(|c| c.get_logger().expect("Logger must be valid")).collect()
    ).unwrap();
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
    info!("Setup complete for {}", config.device);
    let alarms = match setup_metrics(config.alarms) {
        Ok(alarms) => { alarms }
        Err(err) => { panic!("Error occurred while constructing alarms {}", err) }
    };

    let mut scheduler = Scheduler::new();
    for mut alarm in alarms {
        info!("{:?}", alarm);
        scheduler.every(Interval::Minutes(alarm.get_period())).run(move || checker(alarm.borrow_mut()));
    }
    let handle = scheduler.watch_thread(Duration::from_secs(5));
    std::thread::sleep(Duration::from_secs(60 * 7));
    handle.stop()
}

fn checker(alarm: &mut Box<dyn Alarm>) {
    if let Err(e) = alarm.poll_metric() {
        error!("Error occurred while gathering data : {}", e)
    }
    match alarm.check_conditions() {
        Ok(result) => {
            match result {
                AlarmStatus::Ok => {
                    match alarm.previous_status() {
                        AlarmStatus::Ok => { debug!("No change"); }
                        AlarmStatus::Alarm => { info!("State changed from Alarm to Ok"); }
                        AlarmStatus::NoData => { debug!("No change"); }
                    }
                }
                AlarmStatus::Alarm => {
                    match alarm.previous_status() {
                        AlarmStatus::Ok => { info!("State changed from Ok to Alarm"); }
                        AlarmStatus::Alarm => { debug!("No change"); }
                        AlarmStatus::NoData => { debug!("No change"); }
                    }
                }
                AlarmStatus::NoData => {}
            }
            alarm.set_status(result);
        }
        Err(err) => { error!("Error occurred: {}", err) }
    }
}