use core::fmt;
use ldm_metrics::core::config::AlarmConfiguration;
use ldm_notifications::core::config::NotificationConfiguration;
use serde::export::fmt::Debug;
use serde::export::Formatter;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::PathBuf;
use std::str::FromStr;

pub fn get_config_dir() -> Result<PathBuf, std::io::Error> {
    let mut config_dir = match dirs::config_dir() {
        None => {
            return Err(Error::from(ErrorKind::NotFound));
        }
        Some(dir) => dir,
    };

    config_dir.push("ldm/");
    Ok(config_dir.clone())
}

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
