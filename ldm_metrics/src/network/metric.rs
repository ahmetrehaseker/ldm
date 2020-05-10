use crate::core::config::{Alarm, Metric, MetricConfiguration, SampleCollectError};
use std::borrow::BorrowMut;
use systemstat::{Duration, NetworkStats, Platform, System};

fn get_stat(name: String) -> Result<NetworkStats, SampleCollectError> {
    let sys = System::new();
    match sys.network_stats(name.as_str()) {
        Ok(stat) => Ok(stat),
        Err(err) => {
            return Err(SampleCollectError::new(format!(
                "Error while gathering info for Network: {}",
                err
            )))
        }
    }
}

#[derive(Debug)]
pub struct NetworkRxUsageMetric {
    interval: u64,
    alarms: Vec<Alarm>,
    dimension: String,
}

impl NetworkRxUsageMetric {
    pub fn new(config: MetricConfiguration) -> NetworkRxUsageMetric {
        NetworkRxUsageMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
            dimension: config
                .dimension
                .expect("Dimension must be specified in disk metric"),
        }
    }
}

impl Metric for NetworkRxUsageMetric {
    fn get_name(&self) -> String {
        String::from(format!("network::rx::usage::{}", self.dimension))
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        let first = get_stat(self.dimension.clone())?;
        std::thread::sleep(Duration::from_secs(1));
        let second = get_stat(self.dimension.clone())?;
        Ok((second.rx_bytes.0 - first.rx_bytes.0) as f64 / 1000.0)
    }

    fn get_alarms(&mut self) -> &mut [Alarm] {
        self.alarms.borrow_mut()
    }

    fn get_period(&self) -> u32 {
        (self.interval * 60) as u32
    }
}

#[derive(Debug)]
pub struct NetworkTxUsageMetric {
    interval: u64,
    alarms: Vec<Alarm>,
    dimension: String,
}

impl NetworkTxUsageMetric {
    pub fn new(config: MetricConfiguration) -> NetworkTxUsageMetric {
        NetworkTxUsageMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
            dimension: config
                .dimension
                .expect("Dimension must be specified in disk metric"),
        }
    }
}

impl Metric for NetworkTxUsageMetric {
    fn get_name(&self) -> String {
        String::from(format!("network::tx::usage::{}", self.dimension))
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        let first = get_stat(self.dimension.clone())?;
        std::thread::sleep(Duration::from_secs(1));
        let second = get_stat(self.dimension.clone())?;
        Ok((second.tx_bytes.0 - first.tx_bytes.0) as f64 / 1000.0)
    }

    fn get_alarms(&mut self) -> &mut [Alarm] {
        self.alarms.borrow_mut()
    }

    fn get_period(&self) -> u32 {
        (self.interval * 60) as u32
    }
}

#[derive(Debug)]
pub struct NetworkRxTotalMetric {
    interval: u64,
    alarms: Vec<Alarm>,
    dimension: String,
}

impl NetworkRxTotalMetric {
    pub fn new(config: MetricConfiguration) -> NetworkRxTotalMetric {
        NetworkRxTotalMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
            dimension: config
                .dimension
                .expect("Dimension must be specified in disk metric"),
        }
    }
}

impl Metric for NetworkRxTotalMetric {
    fn get_name(&self) -> String {
        String::from(format!("network::rx::total::{}", self.dimension))
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        let stat = get_stat(self.dimension.clone())?;
        Ok((stat.rx_bytes.0) as f64 / 1000.0)
    }

    fn get_alarms(&mut self) -> &mut [Alarm] {
        self.alarms.borrow_mut()
    }

    fn get_period(&self) -> u32 {
        (self.interval * 60) as u32
    }
}

#[derive(Debug)]
pub struct NetworkTxTotalMetric {
    interval: u64,
    alarms: Vec<Alarm>,
    dimension: String,
}

impl NetworkTxTotalMetric {
    pub fn new(config: MetricConfiguration) -> NetworkTxTotalMetric {
        NetworkTxTotalMetric {
            interval: config.interval,
            alarms: Alarm::from(config.alarms),
            dimension: config
                .dimension
                .expect("Dimension must be specified in disk metric"),
        }
    }
}

impl Metric for NetworkTxTotalMetric {
    fn get_name(&self) -> String {
        String::from(format!("network::tx::total::{}", self.dimension))
    }

    fn poll_metric(&mut self) -> Result<f64, SampleCollectError> {
        let stat = get_stat(self.dimension.clone())?;
        Ok((stat.tx_bytes.0) as f64 / 1000.0)
    }

    fn get_alarms(&mut self) -> &mut [Alarm] {
        self.alarms.borrow_mut()
    }

    fn get_period(&self) -> u32 {
        (self.interval * 60) as u32
    }
}
