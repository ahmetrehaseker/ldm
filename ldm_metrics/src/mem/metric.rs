use crate::core::config::{AlarmConfiguration, Alarm};

pub struct MemTotalAlarm {
    config: AlarmConfiguration,
}

impl MemTotalAlarm {
    pub fn new(config: AlarmConfiguration) -> MemTotalAlarm {
        MemTotalAlarm { config }
    }
}

impl Alarm for MemTotalAlarm {
    fn check_conditions(&self) {
        unimplemented!()
    }
}

pub struct MemUsageAlarm {
    config: AlarmConfiguration,
}

impl MemUsageAlarm {
    pub fn new(config: AlarmConfiguration) -> MemUsageAlarm {
        MemUsageAlarm { config }
    }
}

impl Alarm for MemUsageAlarm {
    fn check_conditions(&self) {
        unimplemented!()
    }
}

pub struct MemFreeAlarm {
    config: AlarmConfiguration,
}

impl MemFreeAlarm {
    pub fn new(config: AlarmConfiguration) -> MemFreeAlarm {
        MemFreeAlarm { config }
    }
}

impl Alarm for MemFreeAlarm {
    fn check_conditions(&self) {
        unimplemented!()
    }
}