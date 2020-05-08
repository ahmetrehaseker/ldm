use crate::core::config::{
    Alarm, AlarmCheckError, AlarmConfiguration, AlarmStatus, SampleCollectError,
};
use systemstat::{Duration, Platform, System};

#[derive(Debug)]
pub struct NetworkInAlarm {
    config: AlarmConfiguration,
    samples: Vec<f32>,
    previous_status: AlarmStatus,
}

impl NetworkInAlarm {
    pub fn new(config: AlarmConfiguration) -> NetworkInAlarm {
        NetworkInAlarm {
            config,
            samples: vec![],
            previous_status: AlarmStatus::NoData,
        }
    }
}

impl Alarm for NetworkInAlarm {
    fn check_conditions(&self) -> Result<AlarmStatus, AlarmCheckError> {
        if self.samples.len() < self.config.sample_size() {
            return Ok(AlarmStatus::NoData);
        }
        let mut res = true;
        for cond in self.config.conditions() {
            res = res & cond.check_condition(&self.samples);
        }
        Ok(match res {
            true => AlarmStatus::Ok,
            false => AlarmStatus::Alarm,
        })
    }

    fn poll_metric(&mut self) -> Result<(), SampleCollectError> {
        let sys = System::new();

        if self.samples.len() == self.config.sample_size() {
            self.samples.remove(0);
        }
        if let Some(x) = self.config.dimension() {
            let first = match sys.network_stats(x) {
                Ok(stat) => stat,
                Err(err) => {
                    return Err(SampleCollectError::new(format!(
                        "Error while gathering info for Network: {}",
                        err
                    )))
                }
            };
            std::thread::sleep(Duration::from_secs(1));
            let second = match sys.network_stats(x) {
                Ok(stat) => stat,
                Err(err) => {
                    return Err(SampleCollectError::new(format!(
                        "Error while gathering info for Network: {}",
                        err
                    )))
                }
            };
            self.samples
                .push((second.rx_bytes.0 - first.tx_bytes.0) as f32 / 1000.0)
        }
        Ok(())
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

#[derive(Debug)]
pub struct NetworkOutAlarm {
    config: AlarmConfiguration,
    samples: Vec<f32>,
    previous_status: AlarmStatus,
}

impl NetworkOutAlarm {
    pub fn new(config: AlarmConfiguration) -> NetworkOutAlarm {
        NetworkOutAlarm {
            config,
            samples: vec![],
            previous_status: AlarmStatus::NoData,
        }
    }
}

impl Alarm for NetworkOutAlarm {
    fn check_conditions(&self) -> Result<AlarmStatus, AlarmCheckError> {
        if self.samples.len() < self.config.sample_size() {
            return Ok(AlarmStatus::NoData);
        }
        let mut res = true;
        for cond in self.config.conditions() {
            res = res & cond.check_condition(&self.samples);
        }
        Ok(match res {
            true => AlarmStatus::Ok,
            false => AlarmStatus::Alarm,
        })
    }

    fn poll_metric(&mut self) -> Result<(), SampleCollectError> {
        let sys = System::new();

        if self.samples.len() == self.config.sample_size() {
            self.samples.remove(0);
        }
        if let Some(x) = self.config.dimension() {
            let first = match sys.network_stats(x) {
                Ok(stat) => stat,
                Err(err) => {
                    return Err(SampleCollectError::new(format!(
                        "Error while gathering info for Network: {}",
                        err
                    )))
                }
            };
            std::thread::sleep(Duration::from_secs(1));
            let second = match sys.network_stats(x) {
                Ok(stat) => stat,
                Err(err) => {
                    return Err(SampleCollectError::new(format!(
                        "Error while gathering info for Network: {}",
                        err
                    )))
                }
            };
            self.samples
                .push((second.tx_bytes.0 - first.tx_bytes.0) as f32 / 1000.0)
        }
        Ok(())
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
