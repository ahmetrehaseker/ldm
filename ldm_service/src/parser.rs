use core::fmt;
use ldm_metrics::core::config::AlarmConfiguration;
use ldm_notifications::core::config::NotificationConfiguration;
use serde::export::fmt::Debug;
use serde::export::Formatter;
use serde_derive::Deserialize;
use simplelog::*;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::str::FromStr;

pub fn get_config(name: &str) -> Result<Config, std::io::Error> {
    let mut config_dir = match dirs::config_dir() {
        None => {
            return Err(Error::from(ErrorKind::NotFound));
        }
        Some(dir) => dir,
    };

    config_dir.push(name);
    let mut file = File::open(config_dir)?;
    let mut config_toml = String::new();
    file.read_to_string(&mut config_toml)?;
    let config: Config =
        toml::from_str(&config_toml).unwrap_or_else(|err| panic!("Error: [{}]", err));
    return Ok(config);
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub device: DeviceConf,
    pub logs: Vec<LogConfiguration>,
    pub alarms: Vec<AlarmConfiguration>,
    pub notifications: Vec<NotificationConfiguration>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceConf {
    name: String,
    ip: String,
}

impl fmt::Display for DeviceConf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Device : '{}' [{}]", self.name, self.ip)
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum LogConfiguration {
    #[serde(rename = "console")]
    Console { level: String },
    #[serde(rename = "file")]
    File { level: String, path: String },
}

impl LogConfiguration {
    pub fn get_logger(&self) -> Option<Box<dyn SharedLogger>> {
        match self {
            LogConfiguration::Console { level } => Some(
                TermLogger::new(
                    LevelFilter::from_str(level.as_str())
                        .expect(format!("Level '{}' is not recognized", level).as_str()),
                    simplelog::Config::default(),
                    TerminalMode::Mixed,
                )
                .unwrap(),
            ),
            LogConfiguration::File { level, path } => Some(WriteLogger::new(
                LevelFilter::from_str(level.as_str())
                    .expect(format!("Level '{}' is not recognized", level).as_str()),
                simplelog::Config::default(),
                File::create(path).unwrap(),
            )),
        }
    }
}
