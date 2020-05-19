
pub mod metric;

pub use metric::Metric;
pub use metric::MetricKind;

#[cfg(test)]
mod tests {
    use crate::metric::*;

    #[test]
    fn metric_size() {
        assert_eq!(16, std::mem::size_of::<MetricKind>());
        assert_eq!(48, std::mem::size_of::<Metric>());
    }
}

