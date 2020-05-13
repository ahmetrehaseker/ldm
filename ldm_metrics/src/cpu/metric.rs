use crate::core::config::{Alarm, Metric, MetricConfiguration, SampleCollectError};
use std::borrow::BorrowMut;
use systemstat::{Duration, Platform, System};

#[derive(Debug)]
pub struct CpuUsageMetric {
    interval: u64,
    alarms: Vec<Alarm>,
}

impl CpuUsageMetric {
    pub fn new(config: MetricConfiguration) -> CpuUsageMetric {
        CpuUsageMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
        }
    }
}

impl Metric for CpuUsageMetric {
    fn get_name(&self) -> String {
        String::from("cpu::usage")
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        let sys = System::new();
        match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                std::thread::sleep(Duration::from_secs(1));
                let load = cpu.done().unwrap();
                Ok(load.user as f64)
            }
            Err(err) => Err(SampleCollectError::new(format!(
                "Error while gathering info for Cpu: {}",
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
