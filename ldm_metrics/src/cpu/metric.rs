use crate::core::config::{AlarmConfiguration, Alarm, AlarmCheckError, SampleCollectError, AlarmStatus};
use systemstat::{System, Platform, Duration};

#[derive(Debug)]
pub struct CpuUsedAlarm {
    config: AlarmConfiguration,
    samples: Vec<f32>,
    previous_status: AlarmStatus,
}

impl CpuUsedAlarm {
    pub fn new(config: AlarmConfiguration) -> CpuUsedAlarm {
        CpuUsedAlarm { config, samples: vec![], previous_status: AlarmStatus::NoData }
    }
}

impl Alarm for CpuUsedAlarm {
    fn check_conditions(&self) -> Result<AlarmStatus, AlarmCheckError> {
        if self.samples.len() < self.config.sample_size() { return Ok(AlarmStatus::NoData); }
        let mut res = true;
        for cond in self.config.conditions() {
            res = res & cond.check_condition(&self.samples);
        }
        Ok(match res {
            true => AlarmStatus::Ok,
            false => AlarmStatus::Alarm
        })
    }

    fn poll_metric(&mut self) -> Result<(), SampleCollectError> {
        let sys = System::new();

        if self.samples.len() == self.config.sample_size() {
            self.samples.remove(0);
        }

        match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                std::thread::sleep(Duration::from_secs(1));
                let load = cpu.done().unwrap();
                self.samples.push(load.user)
            }
            Err(err) => {
                error!("Error: {}", err)
            }
        }
        Ok(())
    }

    fn get_period(&self) -> u32 {
        60 * self.config.interval() as u32
    }

    fn current_status(&self) -> &AlarmStatus {
        &self.previous_status
    }

    fn set_status(&mut self, status: AlarmStatus) {
        self.previous_status = status;
    }
}


