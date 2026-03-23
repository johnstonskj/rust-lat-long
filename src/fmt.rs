use crate::inner;
use core::{
    fmt::{Debug, Write},
    hash::Hash,
};
use ordered_float::OrderedFloat;

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

pub trait Formatter {
    fn format<W: Write>(&self, f: &mut W, options: &FormatOptions) -> std::fmt::Result;

    fn to_formatted_string(&self, fmt: &FormatOptions) -> String {
        let mut buffer = String::new();
        self.format(&mut buffer, fmt).unwrap();
        buffer
    }
}

/// The default format is [`FormatKind::Decimal`] (plain decimal degrees).
/// When you use the alternate flag (`{:#}`) the default DMS variant is used.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FormatOptions {
    precision: Option<usize>,
    kind: FormatKind,
    labels: Option<(char, char)>,
}

/// | Variant      | Example              |
/// |--------------|----------------------|
/// | `Decimal`    | `48.8582`            |
/// | `DmsSigned`  | `48° 51′ 29.6″`      |
/// | `DmsLabeled` | `48° 51′ 29.6″ N`    |
/// | `DmsBare`    | `+048:51:29.600000`  |
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatKind {
    #[default]
    /// Decimal degrees, e.g. `48.8582`. This format has a *default* precision of 8 decimal places.
    Decimal,
    /// Degrees with Unicode symbols, e.g. `-48° 51′ 29.600000″`. This format has a *default* precision of 6 decimal places.
    DmsSigned,
    /// Degrees with a cardinal-direction label, e.g. `48° 51′ 29.600000″ N`. This format has a *default* precision of 6 decimal places.
    DmsLabeled,
    /// Degrees with no symbols, e.g. `048:51:29.600000`. This format has a *minimum* precision of 4 decimal places, and a *default* precision of 6.
    DmsBare,
}

// ---------------------------------------------------------------------------
// Public Constants
// ---------------------------------------------------------------------------

pub const DEFAULT_DECIMAL_PRECISION: usize = 8;
pub const DEFAULT_DMS_PRECISION: usize = 6;
pub const MINIMUM_DMS_BARE_PRECISION: usize = 4;

// ---------------------------------------------------------------------------
// Implementations >> Formatter
// ---------------------------------------------------------------------------

impl Formatter for OrderedFloat<f64> {
    fn format<W: Write>(&self, f: &mut W, options: &FormatOptions) -> std::fmt::Result {
        formatter_impl(*self, f, options)
    }
}

// ---------------------------------------------------------------------------
// Implementations >> FormatOptions
// ---------------------------------------------------------------------------

impl From<FormatKind> for FormatOptions {
    fn from(kind: FormatKind) -> Self {
        Self::new(kind)
    }
}

impl FormatOptions {
    const fn new(kind: FormatKind) -> Self {
        Self {
            precision: None,
            kind,
            labels: None,
        }
    }

    /// Return a [`FormatOptions`] for decimal degrees with the default precision.
    pub const fn decimal() -> Self {
        Self::new(FormatKind::Decimal).with_default_precision()
    }

    /// Return a [`FormatOptions`] for degrees, minutes, seconds with the default precision.
    pub const fn dms() -> Self {
        Self::dms_signed()
    }

    /// Return a [`FormatOptions`] for signed degrees, minutes, seconds with the default precision.
    pub const fn dms_signed() -> Self {
        Self::new(FormatKind::DmsSigned).with_default_precision()
    }

    /// Return a [`FormatOptions`] for labeled degrees, minutes, seconds with the default precision.
    pub const fn dms_labeled() -> Self {
        Self::new(FormatKind::DmsLabeled).with_default_precision()
    }

    /// Return a [`FormatOptions`] for bare degrees, minutes, seconds with the default precision.
    pub const fn dms_bare() -> Self {
        Self::new(FormatKind::DmsBare).with_default_precision()
    }

    pub const fn with_precision(mut self, precision: usize) -> Self {
        self.precision = Some(precision);
        self
    }

    pub const fn with_default_precision(mut self) -> Self {
        match self.kind {
            FormatKind::Decimal => self.precision = Some(DEFAULT_DECIMAL_PRECISION),
            _ => self.precision = Some(DEFAULT_DMS_PRECISION),
        }
        self
    }

    pub const fn with_labels(mut self, labels: (char, char)) -> Self {
        self.labels = Some(labels);
        self
    }

    pub const fn with_latitude_labels(mut self) -> Self {
        self.labels = Some(('N', 'S'));
        self
    }

    pub const fn with_longitude_labels(mut self) -> Self {
        self.labels = Some(('E', 'W'));
        self
    }

    pub const fn kind(&self) -> FormatKind {
        self.kind
    }

    pub const fn is_decimal(&self) -> bool {
        matches!(self.kind(), FormatKind::Decimal)
    }

    pub const fn is_dms(&self) -> bool {
        self.is_dms_signed() || self.is_dms_labeled() || self.is_dms_bare()
    }

    pub const fn is_dms_signed(&self) -> bool {
        matches!(self.kind(), FormatKind::DmsSigned)
    }

    pub const fn is_dms_labeled(&self) -> bool {
        matches!(self.kind(), FormatKind::DmsLabeled)
    }

    pub const fn is_dms_bare(&self) -> bool {
        matches!(self.kind(), FormatKind::DmsBare)
    }

    pub const fn precision(&self) -> Option<usize> {
        self.precision
    }

    pub const fn labels(&self) -> Option<(char, char)> {
        self.labels
    }

    pub fn positive_label(&self) -> Option<char> {
        self.labels.as_ref().map(|l| l.0)
    }

    pub fn negative_label(&self) -> Option<char> {
        self.labels.as_ref().map(|l| l.1)
    }
}

// ---------------------------------------------------------------------------
// Internal Functions
// ---------------------------------------------------------------------------

pub(crate) fn formatter_impl<W: Write>(
    angle: OrderedFloat<f64>,
    f: &mut W,
    options: &FormatOptions,
) -> std::fmt::Result {
    match options.kind() {
        FormatKind::Decimal => {
            if let Some(precision) = options.precision() {
                write!(f, "{:.precision$}", angle.into_inner())
            } else {
                write!(f, "{}", angle.into_inner())
            }
        }
        FormatKind::DmsSigned => {
            let (degrees, minutes, seconds) = inner::to_degrees_minutes_seconds(angle);
            if let Some(precision) = options.precision() {
                write!(f, "{degrees}° {minutes}′ {seconds:.precision$}″")
            } else {
                write!(f, "{degrees}° {minutes}′ {seconds}″")
            }
        }
        FormatKind::DmsLabeled => {
            let (degrees, minutes, seconds) = inner::to_degrees_minutes_seconds(angle);
            let (positive, negative) = options.labels().expect("No labels provided");
            if let Some(precision) = options.precision() {
                write!(
                    f,
                    "{}° {}′ {:.precision$}″ {}",
                    degrees.abs(),
                    minutes,
                    seconds,
                    if angle > inner::ZERO {
                        positive.to_string()
                    } else if angle < inner::ZERO {
                        negative.to_string()
                    } else {
                        "".to_string()
                    }
                )
            } else {
                write!(
                    f,
                    "{}° {}′ {}″ {}",
                    degrees.abs(),
                    minutes,
                    seconds,
                    if angle > inner::ZERO {
                        positive.to_string()
                    } else if angle < inner::ZERO {
                        negative.to_string()
                    } else {
                        "".to_string()
                    }
                )
            }
        }
        FormatKind::DmsBare => {
            let (degrees, minutes, seconds) = inner::to_degrees_minutes_seconds(angle);
            let precision = if let Some(precision) = options.precision()
                && precision >= 4
            {
                precision
            } else {
                MINIMUM_DMS_BARE_PRECISION
            };
            let width = precision + 3;
            write!(f, "{degrees:+04}:{minutes:02}:{seconds:0width$.precision$}",)
        }
    }
}

// ---------------------------------------------------------------------------
// Unit Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::fmt::{FormatOptions, Formatter};
    use ordered_float::OrderedFloat;

    #[test]
    fn test_float_to_string_positive() {
        assert_eq!(
            OrderedFloat(45.508333)
                .to_formatted_string(&FormatOptions::decimal().with_precision(6)),
            "45.508333"
        );
    }

    #[test]
    fn test_float_to_string_negative() {
        assert_eq!(
            OrderedFloat(-45.508333)
                .to_formatted_string(&FormatOptions::decimal().with_precision(6)),
            "-45.508333"
        );
    }

    #[test]
    fn test_float_to_string_signed_positive() {
        assert_eq!(
            OrderedFloat(45.508333).to_formatted_string(&FormatOptions::dms_signed()),
            "45° 30′ 29.998800″"
        );
    }

    #[test]
    fn test_float_to_string_signed_negative() {
        assert_eq!(
            OrderedFloat(-45.508333).to_formatted_string(&FormatOptions::dms_signed()),
            "-45° 30′ 29.998800″"
        );
    }

    #[test]
    fn test_float_to_degree_string_labeled_positive() {
        assert_eq!(
            OrderedFloat(45.508333)
                .to_formatted_string(&FormatOptions::dms_labeled().with_latitude_labels()),
            "45° 30′ 29.998800″ N"
        );
    }

    #[test]
    fn test_float_to_string_labeled_negative() {
        assert_eq!(
            OrderedFloat(-45.508333)
                .to_formatted_string(&FormatOptions::dms_labeled().with_latitude_labels()),
            "45° 30′ 29.998800″ S"
        );
    }

    #[test]
    fn test_float_to_string_bare_positive() {
        assert_eq!(
            OrderedFloat(45.508333).to_formatted_string(&FormatOptions::dms_bare()),
            "+045:30:29.998800"
        );
    }

    #[test]
    fn test_float_to_string_bare_negative() {
        assert_eq!(
            OrderedFloat(-45.508333).to_formatted_string(&FormatOptions::dms_bare()),
            "-045:30:29.998800"
        );
    }
}
