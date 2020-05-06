use crate::core::config::{AlarmConfiguration, Alarm};

pub struct TemperatureAlarm {
    config: AlarmConfiguration
}

impl TemperatureAlarm {
    pub fn new(config: AlarmConfiguration) -> TemperatureAlarm {
        TemperatureAlarm { config }
    }
}

impl Alarm for TemperatureAlarm {
    fn check_conditions(&self) {
        unimplemented!()
    }
}