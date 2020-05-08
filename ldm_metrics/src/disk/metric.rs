use crate::core::config::{
    Alarm, AlarmCheckError, AlarmConfiguration, AlarmStatus, SampleCollectError,
};
use systemstat::{Platform, System};

#[derive(Debug)]
pub struct DiskUsedAlarm {
    config: AlarmConfiguration,
    samples: Vec<f32>,
    previous_status: AlarmStatus,
}

impl DiskUsedAlarm {
    pub fn new(config: AlarmConfiguration) -> DiskUsedAlarm {
        DiskUsedAlarm {
            config,
            samples: vec![],
            previous_status: AlarmStatus::NoData,
        }
    }
}

impl Alarm for DiskUsedAlarm {
    fn check_conditions(&self) -> Result<AlarmStatus, AlarmCheckError> {
        if self.samples.len() < self.config.sample_size() {
            return Ok(AlarmStatus::NoData);
        }
        let mut res = true;
        for cond in self.config.conditions() {
            res = res & cond.check_condition(&self.samples);
        }
        Ok(match res {
            true => AlarmStatus::Alarm,
            false => AlarmStatus::Ok,
        })
    }

    fn poll_metric(&mut self) -> Result<(), SampleCollectError> {
        let sys = System::new();

        if self.samples.len() == self.config.sample_size() {
            self.samples.remove(0);
        }
        if let Some(dimension) = self.config.dimension() {
            match sys.mounts() {
                Ok(mounts) => {
                    for mount in mounts {
                        if dimension.eq(mount.fs_mounted_on.as_str()) {
                            let total = mount.total.as_u64() as f32;
                            let avail = mount.avail.as_u64() as f32;
                            let usage = 100.0 - 100.0 * avail / total;
                            self.samples.push(usage);
                            return Ok(());
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
        }
        Err(SampleCollectError::new(String::from(
            "Dimension(mount path) must be specified",
        )))
    }

    fn get_period(&self) -> u32 {
        60 * self.config.interval() as u32
    }

    fn previous_status(&self) -> &AlarmStatus {
        &self.previous_status
    }

    fn set_status(&mut self, status: AlarmStatus) {
        self.previous_status = status;
    }

    fn get_message(&self) -> String {
        String::from(self.config.message())
    }
}
