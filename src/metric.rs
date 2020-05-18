
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Metric {
    name: String,
    value: f64,
    kind: MetricKind,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum MetricKind {
    Counter(f64), // TODO: What level of precision is needed for this?
    Gauge,
    Timer,

    // Not sure if I need these
    Histogram,
    Meter,

    Unimplemented, // Temporary variant for testing
}

