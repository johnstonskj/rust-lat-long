//! Formatting primitives for geographic angles.
//!
//! This module exposes the [`Formatter`] trait, the [`FormatOptions`] builder,
//! and the [`FormatKind`] enum that together drive how [`crate::Latitude`],
//! [`crate::Longitude`], and [`crate::Coordinate`] values are rendered.
//!
//! Four output styles are supported (see [`FormatKind`] for examples):
//!
//! * [`FormatKind::Decimal`] — plain decimal degrees, e.g. `48.858222`.
//! * [`FormatKind::DmsSigned`] — DMS with a leading sign, e.g. `48° 51′ 29.6″`.
//! * [`FormatKind::DmsLabeled`] — DMS with a cardinal label, e.g. `48° 51′ 29.6″ N`.
//! * [`FormatKind::DmsBare`] — fixed-width DMS without symbols, e.g. `+048:51:29.600000`.
//!
//! The [`Display`](core::fmt::Display) implementations on the public types
//! delegate to [`Formatter::format`] with sensible defaults. Use
//! [`Formatter::to_formatted_string`] when you need an owned `String` and want
//! to pick a non-default style or precision.
//!
//! # Examples
//!
//! ```rust
//! use lat_long::{Angle, Latitude, fmt::{FormatOptions, Formatter}};
//!
//! let lat = Latitude::new(48, 51, 29.6).unwrap();
//! let dms = lat.to_formatted_string(&FormatOptions::dms_labeled().with_latitude_labels());
//! assert!(dms.ends_with(" N"));
//! ```
//! 

use crate::inner;
use core::{
    fmt::{Debug, Write},
    hash::Hash,
};
use ordered_float::OrderedFloat;

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

///
/// Common interface for writing a value to a [`Write`] target using a
/// [`FormatOptions`] descriptor.
///
/// Implementations exist for [`OrderedFloat<f64>`] (the underlying numeric
/// representation of an angle) and for each public coordinate type. Most
/// callers will reach for [`Formatter::to_formatted_string`] when they want
/// an owned `String`.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Angle, Longitude, fmt::{FormatOptions, Formatter}};
///
/// let lon = Longitude::new(-122, 19, 59.0).unwrap();
/// let bare = lon.to_formatted_string(&FormatOptions::dms_bare());
/// assert!(bare.starts_with('-'));
/// ```
///
pub trait Formatter {
    ///
    ///  Write `self` to `f` according to `options`.
    ///
    /// Implementations should respect every relevant field of `options`
    /// (kind, precision, labels) and produce no extraneous whitespace.
    ///
    fn format<W: Write>(&self, f: &mut W, options: &FormatOptions) -> std::fmt::Result;

    /// Convenience helper: render `self` into a new `String`.
    ///
    /// This is implemented in terms of [`Formatter::format`] and a `String`
    /// buffer, so it cannot fail in practice.
    fn to_formatted_string(&self, fmt: &FormatOptions) -> String {
        let mut buffer = String::new();
        self.format(&mut buffer, fmt).unwrap();
        buffer
    }
}

///
/// The default format is [`FormatKind::Decimal`] (plain decimal degrees).
/// When you use the alternate flag (`{:#}`) the default DMS variant is used.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FormatOptions {
    precision: Option<usize>,
    kind: FormatKind,
    labels: Option<(char, char)>,
}

///
/// | Variant      | Example              |
/// |--------------|----------------------|
/// | `Decimal`    | `48.8582`            |
/// | `DmsSigned`  | `48° 51′ 29.6″`      |
/// | `DmsLabeled` | `48° 51′ 29.6″ N`    |
/// | `DmsBare`    | `+048:51:29.600000`  |
///
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatKind {
    #[default]
    /// 
    /// Format as decimal degrees, e.g. `48.8582`. This format has a *default* precision
    /// of 8 decimal places.
    /// 
    Decimal,

    /// Format as degrees with Unicode symbols, e.g. `-48° 51′ 29.600000″`. This format
    /// has a *default* precision of 6 decimal places.
    DmsSigned,

    /// Format as degrees with a cardinal-direction label, e.g. `48° 51′ 29.600000″ N`.
    /// This format has a *default* precision of 6 decimal places.
    DmsLabeled,

    /// Format as degrees with no symbols, e.g. `048:51:29.600000`. This format has a
    /// *minimum* precision of 4 decimal places, and a *default* precision of 6.
    DmsBare,
}

// ---------------------------------------------------------------------------
// Public Constants
// ---------------------------------------------------------------------------

/// 
/// Default number of fractional digits used when rendering decimal degrees.
/// 
pub const DEFAULT_DECIMAL_PRECISION: usize = 8;

/// 
/// Default number of fractional digits used for the seconds component of any
/// DMS rendering.
/// 
pub const DEFAULT_DMS_PRECISION: usize = 6;

/// 
/// Minimum number of fractional digits enforced by [`FormatKind::DmsBare`].
///
/// The bare format is designed to be machine-parseable, so its seconds field
/// has a guaranteed-minimum width regardless of the requested precision.
/// 
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

    /// 
    /// Return a [`FormatOptions`] for decimal degrees with the default precision.
    /// 
    pub const fn decimal() -> Self {
        Self::new(FormatKind::Decimal).with_default_precision()
    }

    /// 
    /// Return a [`FormatOptions`] for degrees, minutes, seconds with the default precision.
    /// 
    pub const fn dms() -> Self {
        Self::dms_signed()
    }

    /// 
    /// Return a [`FormatOptions`] for signed degrees, minutes, seconds with the default precision.
    /// 
    pub const fn dms_signed() -> Self {
        Self::new(FormatKind::DmsSigned).with_default_precision()
    }

    /// 
    /// Return a [`FormatOptions`] for labeled degrees, minutes, seconds with the default precision.
    /// 
    pub const fn dms_labeled() -> Self {
        Self::new(FormatKind::DmsLabeled).with_default_precision()
    }

    /// 
    /// Return a [`FormatOptions`] for bare degrees, minutes, seconds with the default precision.
    /// 
    pub const fn dms_bare() -> Self {
        Self::new(FormatKind::DmsBare).with_default_precision()
    }

    /// 
    /// Override the number of fractional digits used when rendering.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Latitude, fmt::{FormatOptions, Formatter}};
    ///
    /// let lat = Latitude::new(48, 51, 29.6).unwrap();
    /// let s = lat.to_formatted_string(&FormatOptions::decimal().with_precision(2));
    /// assert_eq!(s, "48.86");
    /// ```
    /// 
    pub const fn with_precision(mut self, precision: usize) -> Self {
        self.precision = Some(precision);
        self
    }

    /// 
    /// Set the precision to the default value for the current [`FormatKind`].
    ///
    /// Decimal uses [`DEFAULT_DECIMAL_PRECISION`]; every DMS variant uses
    /// [`DEFAULT_DMS_PRECISION`].
    /// 
    pub const fn with_default_precision(mut self) -> Self {
        match self.kind {
            FormatKind::Decimal => self.precision = Some(DEFAULT_DECIMAL_PRECISION),
            _ => self.precision = Some(DEFAULT_DMS_PRECISION),
        }
        self
    }

    /// 
    /// Set the `(positive, negative)` label pair used by [`FormatKind::DmsLabeled`].
    ///
    /// Prefer [`with_latitude_labels`](Self::with_latitude_labels) or
    /// [`with_longitude_labels`](Self::with_longitude_labels) for the standard
    /// `N`/`S` and `E`/`W` pairs.
    /// 
    pub const fn with_labels(mut self, labels: (char, char)) -> Self {
        self.labels = Some(labels);
        self
    }

    /// 
    /// Convenience: set labels to the latitude pair `('N', 'S')`.
    /// 
    pub const fn with_latitude_labels(mut self) -> Self {
        self.labels = Some(('N', 'S'));
        self
    }

    /// 
    /// Convenience: set labels to the longitude pair `('E', 'W')`.
    /// 
    pub const fn with_longitude_labels(mut self) -> Self {
        self.labels = Some(('E', 'W'));
        self
    }

    /// 
    /// Returns the configured [`FormatKind`].
    /// 
    pub const fn kind(&self) -> FormatKind {
        self.kind
    }

    /// 
    /// Returns `true` if this is a [`FormatKind::Decimal`] format.
    /// 
    pub const fn is_decimal(&self) -> bool {
        matches!(self.kind(), FormatKind::Decimal)
    }

    /// 
    /// Returns `true` if this is any DMS variant (signed, labeled, or bare).
    /// 
    pub const fn is_dms(&self) -> bool {
        self.is_dms_signed() || self.is_dms_labeled() || self.is_dms_bare()
    }

    /// 
    /// Returns `true` if this is the [`FormatKind::DmsSigned`] variant.
    /// 
    pub const fn is_dms_signed(&self) -> bool {
        matches!(self.kind(), FormatKind::DmsSigned)
    }

    /// 
    /// Returns `true` if this is the [`FormatKind::DmsLabeled`] variant.
    /// 
    pub const fn is_dms_labeled(&self) -> bool {
        matches!(self.kind(), FormatKind::DmsLabeled)
    }

    /// 
    /// Returns `true` if this is the [`FormatKind::DmsBare`] variant.
    /// 
    pub const fn is_dms_bare(&self) -> bool {
        matches!(self.kind(), FormatKind::DmsBare)
    }

    /// 
    /// Returns the configured precision, if any was set.
    /// 
    pub const fn precision(&self) -> Option<usize> {
        self.precision
    }

    /// 
    /// Returns the configured `(positive, negative)` label pair, if any.
    /// 
    pub const fn labels(&self) -> Option<(char, char)> {
        self.labels
    }

    /// 
    /// Returns just the label used for positive values, if labels are set.
    /// 
    pub fn positive_label(&self) -> Option<char> {
        self.labels.as_ref().map(|l| l.0)
    }

    /// 
    /// Returns just the label used for negative values, if labels are set.
    /// 
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
