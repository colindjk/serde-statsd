
#[allow(dead_code)]
pub(crate) enum ErrorCode {
    /// Catchall for syntax error messages
    Message(Box<str>),

    /// Some IO error occurred while serializing or deserializing.
    Io(io::Error),

    /// EOF while parsing value section of a Metric
    EofWhileParsingMetricValue,

    /// After 'c|', expected a rate, found EOF / invalid value
    EofWhileParsingCounterRate,

    /// EOF while parsing metric type
    EofWhileParsingMetricType,

    /// After ':', expected number value.
    ExpectedMetricValue,

    /// Expected this to be a known metric type, found invalid type
    ExpectedMetricType,

    /// After 'c|', expected a rate, found EOF / invalid value
    ExpectedCounterRate,

    /// Number is bigger than the maximum value of its type.
    NumberOutOfRange,

    /// Metric has trailing characters
    TrailingCharacters,

    /// JSON has non-whitespace trailing characters after the value.
    TrailingCharacters,
}

/// Boxed error code to preserve space TODO: Is this necessary?
pub struct Error(Box<ErrorCode>);

