use crate::core::config::{Alarm, Metric, MetricConfiguration, SampleCollectError};
use std::borrow::BorrowMut;
use systemstat::{Platform, System};

#[derive(Debug)]
pub struct TemperatureMetric {
    interval: u64,
    alarms: Vec<Alarm>,
}

impl TemperatureMetric {
    pub fn new(config: MetricConfiguration) -> TemperatureMetric {
        TemperatureMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
        }
    }
}

impl Metric for TemperatureMetric {
    fn get_name(&self) -> String {
        String::from("temperature")
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        match System::new().cpu_temp() {
            Ok(temp) => Ok(temp as f64),
            Err(err) => Err(SampleCollectError::new(format!(
                "Error while gathering info for Temperature: {}",
                err
            ))),
        }
    }

    fn get_alarms(&mut self) -> &mut [Alarm] {
        self.alarms.borrow_mut()
    }

    fn get_period(&self) -> u32 {
        (self.interval * 60) as u32
    }
}
