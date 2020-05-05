#[macro_use]
extern crate log;
extern crate simplelog;
extern crate ldm_commons;

use std::time::Duration;
use ldm_service::parser::{LogConfiguration, get_config};
use simplelog::CombinedLogger;

pub fn init_logger(conf: Vec<LogConfiguration>) {
    CombinedLogger::init(
        conf.iter().map(|c| c.get_logger().expect("Logger must be valid")).collect()
    ).unwrap();
}

fn main() {
    // init_logger();
    let config = match get_config("ldm/config.toml") {
        Ok(config) => config,
        Err(err) => {
            panic!("Error occurred while gating config.toml, {}", err);
        }
    };
    init_logger(config.logs);
    info!("Setup complete for {}", config.device);
    std::thread::sleep(Duration::from_secs(60))
}
