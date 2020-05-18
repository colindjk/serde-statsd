
mod metric;


#[cfg(test)]
mod tests {
    use crate::metric::*;

    #[test]
    fn it_works() {
        assert_eq!(16, std::mem::size_of::<MetricKind>());
        assert_eq!(16, std::mem::size_of::<Metric>());
    }
}

