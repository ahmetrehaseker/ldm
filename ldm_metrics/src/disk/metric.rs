use crate::core::config::{AlarmConfiguration, Alarm, AlarmCheckError, SampleCollectError};
use heim::{disk, units, Error};
use tokio::stream::StreamExt;

pub struct DiskUsedAlarm {
    config: AlarmConfiguration,
    samples: Vec<f32>,
}

impl DiskUsedAlarm {
    pub fn new(config: AlarmConfiguration) -> DiskUsedAlarm {
        DiskUsedAlarm { config, samples: vec![] }
    }
}

impl Alarm for DiskUsedAlarm {
    fn check_conditions(&self) -> Result<bool, AlarmCheckError> {
        Ok(self.config.conditions().iter().map(|c| c.check_condition(&self.samples)).all(true))
    }

    async fn get_sample(&mut self) -> Result<(), SampleCollectError> {
        if self.samples.len() == self.config.sample_size() {
            self.samples.remove(0);
        }
        let mut partitions = disk::partitions_physical();
        tokio::pin!(partitions);
        while let Some(part) = partitions.next().await {
            if let Some(part) = part {
                if let Some(usage)  = heim::disk::usage(part.mount_point().to_path_buf()).await {

                }
                
                println!(
                    "{:<17} {:<10} {:<10} {:<10} {:<10} {}",
                    part.device()
                        .unwrap_or_else(|| OsStr::new("N/A"))
                        .to_string_lossy(),
                    usage.total().get::<units::information::megabyte>(),
                    usage.used().get::<units::information::megabyte>(),
                    usage.free().get::<units::information::megabyte>(),
                    part.file_system().as_str(),
                    part.mount_point().to_string_lossy(),
                );
            }
        }
        Ok(())
    }
}

pub struct DiskFreeAlarm {
    config: AlarmConfiguration,
    samples: Vec<f32>,
}

impl DiskFreeAlarm {
    pub fn new(config: AlarmConfiguration) -> DiskFreeAlarm {
        DiskFreeAlarm { config, samples: vec![] }
    }
}

impl Alarm for DiskFreeAlarm {
    fn check_conditions(&self) -> Result<bool, AlarmCheckError> {
        Ok(self.config.conditions().iter().map(|c| c.check_condition(&self.samples)).all(true))
    }

    async fn get_sample(&mut self) -> Result<(), SampleCollectError> {
        unimplemented!()
    }
}