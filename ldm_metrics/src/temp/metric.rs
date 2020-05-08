use crate::core::config::{
    Alarm, AlarmCheckError, AlarmConfiguration, AlarmStatus, SampleCollectError,
};
use systemstat::{Platform, System};

#[derive(Debug)]
pub struct TemperatureAlarm {
    config: AlarmConfiguration,
    samples: Vec<f32>,
    previous_status: AlarmStatus,
}

impl TemperatureAlarm {
    pub fn new(config: AlarmConfiguration) -> TemperatureAlarm {
        TemperatureAlarm {
            config,
            samples: vec![],
            previous_status: AlarmStatus::NoData,
        }
    }
}

impl Alarm for TemperatureAlarm {
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
        match sys.cpu_temp() {
            Ok(temp) => self.samples.push(temp),
            Err(err) => {
                return Err(SampleCollectError::new(format!(
                    "Error while gathering info for Temperature: {}",
                    err
                )))
            }
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
