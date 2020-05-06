use crate::core::config::{AlarmConfiguration, Alarm};
use crate::cpu::metric::CpuUsedAlarm;
// use crate::disk::metric::{DiskFreeAlarm, DiskUsedAlarm};
// use crate::mem::metric::{MemUsageAlarm, MemTotalAlarm, MemFreeAlarm};
// use crate::temp::metric::TemperatureAlarm;
// use crate::network::metric::{NetworkInAlarm, NetworkOutAlarm, NetworkTotalAlarm};
use core::fmt;

pub fn setup_metrics(configurations: Vec<AlarmConfiguration>) -> Result<Vec<Box<dyn Alarm>>, MetricNotFoundError> {
    let mut metrics: Vec<Box<dyn Alarm>> = Vec::new();
    for conf in configurations {
        metrics.push(setup_metric(conf)?)
    }
    Ok(metrics)
}

pub fn setup_metric(configuration: AlarmConfiguration) -> Result<Box<dyn Alarm>, MetricNotFoundError> {
    match configuration.kind() {
        "cpu::used" => Ok(Box::new(CpuUsedAlarm::new(configuration))),
        // "mem::usage" => Ok(Box::new(MemUsageAlarm::new(configuration))),
        // "mem::total" => Ok(Box::new(MemTotalAlarm::new(configuration))),
        // "mem::free" => Ok(Box::new(MemFreeAlarm::new(configuration))),
        // "disk::used" => Ok(Box::new(DiskUsedAlarm::new(configuration))),
        // "disk::free" => Ok(Box::new(DiskFreeAlarm::new(configuration))),
        // "network::in" => Ok(Box::new(NetworkInAlarm::new(configuration))),
        // "network::out" => Ok(Box::new(NetworkOutAlarm::new(configuration))),
        // "network::total" => Ok(Box::new(NetworkTotalAlarm::new(configuration))),
        // "temp" => Ok(Box::new(TemperatureAlarm::new(configuration))),
        _ => Err(MetricNotFoundError { metric: String::from(configuration.kind()) })
    }
}

pub enum MetricTypes {
    CpuUsed(CpuUsedAlarm),

    // MemUsed(MemUsageAlarm),
    // MemTotal(MemTotalAlarm),
    // MemFree(MemFreeAlarm),
    //
    // DiskUsed(DiskUsedAlarm),
    // DiskFree(DiskFreeAlarm),
    //
    // NetworkIn(NetworkInAlarm),
    // NetworkOut(NetworkOutAlarm),
    // NetworkTotal(NetworkTotalAlarm),
    //
    // Temp(TemperatureAlarm),
}

#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq)]
pub struct MetricNotFoundError {
    metric: String,
}

impl fmt::Display for MetricNotFoundError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(format!("Metric not found {}", self.metric).as_str())
    }
}

// The Error trait is not available in libcore
#[cfg(feature = "std")]
impl error::Error for MetricNotFoundError {
    fn description(&self) -> &str { format!("Metric not found {}", self.metric).as_str() }
}