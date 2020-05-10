use crate::core::config::{Alarm, Metric, MetricConfiguration, SampleCollectError};
use std::borrow::BorrowMut;
use systemstat::{Platform, System};

#[derive(Debug)]
pub struct DiskUsageMetric {
    interval: u64,
    dimension: String,
    alarms: Vec<Alarm>,
}

impl DiskUsageMetric {
    pub fn new(config: MetricConfiguration) -> DiskUsageMetric {
        DiskUsageMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
            dimension: config
                .dimension
                .expect("Dimension must be specified in disk metric"),
        }
    }
}

impl Metric for DiskUsageMetric {
    fn get_name(&self) -> String {
        String::from(format!("disk::usage::{}", self.dimension))
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        let sys = System::new();
        match sys.mounts() {
            Ok(mounts) => {
                for mount in mounts {
                    if self.dimension.eq(mount.fs_mounted_on.as_str()) {
                        let total = mount.total.as_u64() as f64;
                        let avail = mount.avail.as_u64() as f64;
                        let usage = 100.0 - 100.0 * avail / total;
                        return Ok(usage);
                    }
                }
            }
            Err(err) => {
                return Err(SampleCollectError::new(format!(
                    "Error while gathering info for Disk: {}",
                    err
                )))
            }
        }
        Err(SampleCollectError::new(String::from(
            "Given mount path is not valid",
        )))
    }

    fn get_alarms(&mut self) -> &mut [Alarm] {
        self.alarms.borrow_mut()
    }

    fn get_period(&self) -> u32 {
        (self.interval * 60) as u32
    }
}
