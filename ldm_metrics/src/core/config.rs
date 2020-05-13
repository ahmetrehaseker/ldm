use core::fmt;
use serde_derive::Deserialize;
use std::fmt::Debug;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.message)
    }
}

// The Error trait is not available in libcore
#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

pub type AlarmCheckError = Error;
pub type SampleCollectError = Error;
pub type CalculateError = Error;

pub trait Metric: Debug + Send + Sync {
    fn get_name(&self) -> String;
    fn poll_metric(&mut self) -> Result<f64, SampleCollectError>;
    fn get_alarms(&mut self) -> &mut [Alarm];
    fn get_period(&self) -> u32;
}

// pub trait Alarm: Debug + Send + Sync {
//     fn check_conditions(&self) -> Result<AlarmStatus, AlarmCheckError>;
//     fn poll_metric(&mut self) -> Result<(), SampleCollectError>;
//     fn get_period(&self) -> u32;
//     fn previous_status(&self) -> &AlarmStatus;
//     fn set_status(&mut self, status: AlarmStatus);
//     fn get_message(&self) -> String;
// }

#[derive(Deserialize, Debug, Clone)]
pub struct Alarm {
    pub config: AlarmConfiguration,
    pub samples: Vec<f64>,
    pub previous_status: AlarmStatus,
}

impl Alarm {
    pub fn new(config: AlarmConfiguration) -> Alarm {
        Alarm {
            config,
            samples: Vec::new(),
            previous_status: AlarmStatus::NoData,
        }
    }

    pub fn from(configs: Vec<AlarmConfiguration>) -> Vec<Alarm> {
        configs.iter().map(|c| Alarm::new(c.clone())).collect()
    }

    pub fn check(&mut self, data: f64) -> Result<AlarmStatus, AlarmCheckError> {
        if self.samples.len() == self.config.sample_size {
            self.samples.remove(0);
        }
        self.samples.push(data);

        if self.samples.len() < self.config.sample_size {
            return Ok(AlarmStatus::NoData);
        }
        let mut res = true;
        for cond in &self.config.conditions {
            res = res & cond.check_condition(&self.samples);
        }
        Ok(match res {
            true => AlarmStatus::Alarm,
            false => AlarmStatus::Ok,
        })
    }

    pub fn set_status(&mut self, status: AlarmStatus) {
        self.previous_status = status;
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MetricConfiguration {
    pub name: String,
    pub dimension: Option<String>,
    pub alarms: Vec<AlarmConfiguration>,
    pub interval: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AlarmConfiguration {
    name: String,
    severity: AlarmSeverity,
    conditions: Vec<ConditionConfiguration>,
    sample_size: usize,
}

impl AlarmConfiguration {
    pub fn sample_size(&self) -> usize {
        self.sample_size
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn severity(&self) -> String {
        self.severity.get_name()
    }
    pub fn conditions(&self) -> &Vec<ConditionConfiguration> {
        &self.conditions
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum AlarmStatus {
    Ok,
    Alarm,
    NoData,
}

#[derive(Deserialize, Debug, Clone)]
pub enum AlarmSeverity {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}

impl AlarmSeverity {
    pub fn get_name(&self) -> String {
        match self {
            AlarmSeverity::High => String::from("high"),
            AlarmSeverity::Low => String::from("low"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConditionConfiguration {
    comparison: Comparison,
    value: f64,
    method: CalculationMethod,
}

impl ConditionConfiguration {
    pub fn comparison(&self) -> &Comparison {
        &self.comparison
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn method(&self) -> &CalculationMethod {
        &self.method
    }
}

impl ConditionConfiguration {
    pub fn check_condition(&self, data_set: &Vec<f64>) -> bool {
        let actual_data = self.method.calculate(data_set);
        self.comparison.compare(actual_data, self.value)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Comparison {
    #[serde(rename = "g")]
    Greater,
    #[serde(rename = "ge")]
    GreaterAndEqual,
    #[serde(rename = "l")]
    Lesser,
    #[serde(rename = "le")]
    LesserAndEqual,
    #[serde(rename = "e")]
    Equal,
    #[serde(rename = "n")]
    NotEqual,
}

impl Comparison {
    fn compare(&self, actual: f64, expected: f64) -> bool {
        match self {
            Comparison::Greater => actual > expected,
            Comparison::GreaterAndEqual => actual >= expected,
            Comparison::Lesser => actual < expected,
            Comparison::LesserAndEqual => actual <= expected,
            Comparison::Equal => actual == expected,
            Comparison::NotEqual => actual != expected,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum CalculationMethod {
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
}

impl CalculationMethod {
    fn calculate(&self, data_set: &Vec<f64>) -> f64 {
        match self {
            CalculationMethod::Sum => data_set.iter().sum::<f64>(),
            CalculationMethod::Max => data_set
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                .clone(),
            CalculationMethod::Min => data_set
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                .clone(),
            CalculationMethod::Avg => data_set.iter().sum::<f64>() / data_set.len() as f64,
        }
    }
}
