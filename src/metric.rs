use serde::{ Serialize, Deserialize };

/// The Metric struct
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    name: String,
    value: f64,
    kind: MetricKind,
}

/// Implementation of the Metric type, which provides some simple helpers that
/// to make usage of the Metric type easier. 
impl Metric {

    pub fn new(name: String, value: f64, kind: MetricKind) -> Metric {
        Metric { name, value, kind }
    }

    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn get_name_slice(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn get_value(&self) -> f64 {
        self.value
    }

    #[inline]
    pub fn get_kind(&self) -> MetricKind {
        self.kind
    }

    #[inline]
    pub fn is_counter(&self) -> bool {
        match self.kind {
            MetricKind::Counter(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_gauge(&self) -> bool {
        self.kind == MetricKind::Gauge
    }

    #[inline]
    pub fn is_timer(&self) -> bool {
        self.kind == MetricKind::Timer
    }

    #[inline]
    pub fn is_histogram(&self) -> bool {
        self.kind == MetricKind::Histogram
    }

    #[inline]
    pub fn is_meter(&self) -> bool {
        self.kind == MetricKind::Meter
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum MetricKind {
    Counter(f64),
    Gauge,
    Timer,

    Histogram,
    Meter,
}

