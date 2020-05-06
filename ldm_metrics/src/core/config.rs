use serde_derive::Deserialize;
use core::fmt;
use serde::export::fmt::Debug;

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
    fn description(&self) -> &str { self.message.as_str() }
}

pub type AlarmCheckError = Error;
pub type SampleCollectError = Error;
pub type CalculateError = Error;

#[derive(Debug)]
pub enum AlarmStatus {
    Ok,
    Alarm,
    NoData,
}

pub trait Alarm: Debug + Send + Sync {
    fn check_conditions(&self) -> Result<AlarmStatus, AlarmCheckError>;
    fn poll_metric(&mut self) -> Result<(), SampleCollectError>;
    fn get_period(&self) -> u32;
    fn current_status(&self) -> &AlarmStatus;
    fn set_status(&mut self, status: AlarmStatus);
}

#[derive(Deserialize, Debug)]
pub struct AlarmConfiguration {
    message: String,
    kind: String,
    dimension: Option<String>,
    conditions: Vec<ConditionConfiguration>,
    interval: u64,
    sample_size: usize,
}

impl AlarmConfiguration {
    pub fn sample_size(&self) -> usize { self.sample_size }
    pub fn interval(&self) -> u64 { self.interval }
    pub fn kind(&self) -> &str { self.kind.as_str() }
    pub fn dimension(&self) -> &Option<String> { &self.dimension }
    pub fn message(&self) -> &str { self.message.as_str() }
    pub fn conditions(&self) -> &Vec<ConditionConfiguration> { &self.conditions }
}


#[derive(Deserialize, Debug)]
pub struct ConditionConfiguration {
    comparison: Comparison,
    value: f32,
    method: CalculationMethod,
}

impl ConditionConfiguration {
    pub fn comparison(&self) -> &Comparison { &self.comparison }
    pub fn value(&self) -> f32 { self.value }
    pub fn method(&self) -> &CalculationMethod { &self.method }
}

impl ConditionConfiguration {
    pub fn check_condition(&self, data_set: &Vec<f32>) -> bool {
        let actual_data = self.method.calculate(data_set);
        self.comparison.compare(actual_data, self.value)
    }
}

#[derive(Deserialize, Debug)]
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
    fn compare(&self, actual: f32, expected: f32) -> bool {
        match self {
            Comparison::Greater => { actual > expected }
            Comparison::GreaterAndEqual => { actual >= expected }
            Comparison::Lesser => { actual < expected }
            Comparison::LesserAndEqual => { actual <= expected }
            Comparison::Equal => { actual == expected }
            Comparison::NotEqual => { actual != expected }
        }
    }
}

#[derive(Deserialize, Debug)]
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
    fn calculate(&self, data_set: &Vec<f32>) -> f32 {
        match self {
            CalculationMethod::Sum => { data_set.iter().sum::<f32>() }
            CalculationMethod::Max => { data_set.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone() }
            CalculationMethod::Min => { data_set.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone() }
            CalculationMethod::Avg => { data_set.iter().sum::<f32>() / data_set.len() as f32 }
        }
    }
}