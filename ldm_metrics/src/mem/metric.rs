use crate::core::config::{Alarm, Metric, MetricConfiguration, SampleCollectError};
use std::borrow::BorrowMut;
use systemstat::{Platform, System};

#[derive(Debug)]
pub struct MemoryUsageMetric {
    interval: u64,
    alarms: Vec<Alarm>,
}

impl MemoryUsageMetric {
    pub fn new(config: MetricConfiguration) -> MemoryUsageMetric {
        MemoryUsageMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
        }
    }
}

impl Metric for MemoryUsageMetric {
    fn get_name(&self) -> String {
        String::from("memory::usage")
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        return match System::new().memory() {
            Ok(memory) => {
                let total = memory.total.0;
                let free = memory.free.0;
                let used = total - free;
                Ok(100.0 * used as f64 / total as f64)
            }
            Err(err) => Err(SampleCollectError::new(format!(
                "Error while gathering info for Memory: {}",
                err
            ))),
        };
    }

    fn get_alarms(&mut self) -> &mut [Alarm] {
        self.alarms.borrow_mut()
    }

    fn get_period(&self) -> u32 {
        (self.interval * 60) as u32
    }
}
