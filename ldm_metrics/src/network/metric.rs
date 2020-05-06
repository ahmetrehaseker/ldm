use crate::core::config::{AlarmConfiguration, Alarm};

pub struct NetworkInAlarm {
    config: AlarmConfiguration,
}

impl NetworkInAlarm {
    pub fn new(config: AlarmConfiguration) -> NetworkInAlarm {
        NetworkInAlarm { config }
    }
}

impl Alarm for NetworkInAlarm {
    fn check_conditions(&self) {
        unimplemented!()
    }
}

pub struct NetworkOutAlarm {
    config: AlarmConfiguration,
}

impl NetworkOutAlarm {
    pub fn new(config: AlarmConfiguration) -> NetworkOutAlarm {
        NetworkOutAlarm { config }
    }
}

impl Alarm for NetworkOutAlarm {
    fn check_conditions(&self) {
        unimplemented!()
    }
}

pub struct NetworkTotalAlarm {
    config: AlarmConfiguration,
}

impl NetworkTotalAlarm {
    pub fn new(config: AlarmConfiguration) -> NetworkTotalAlarm {
        NetworkTotalAlarm { config }
    }
}

impl Alarm for NetworkTotalAlarm {
    fn check_conditions(&self) {
        unimplemented!()
    }
}