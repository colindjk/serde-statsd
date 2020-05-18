use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    name: String,
    value: f64,
    kind: MetricKind,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MetricKind {
    Counter(f64),
    Gauge,
    Timer,

    Histogram,
    Meter,
}

