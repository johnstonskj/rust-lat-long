//! This module provides the [`Error`] type.

use core::fmt::Display;

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

/// Errors that can occur when constructing or parsing coordinate values.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Generic angle out of range; used internally before conversion to a typed error.
    InvalidAngle(f64, f64),
    /// Latitude degrees outside the valid range −90..=90.
    InvalidLatitudeDegrees(i32),
    /// Longitude degrees outside the valid range −180..=180.
    InvalidLongitudeDegrees(i32),
    /// Minutes value ≥ 60 (valid range is 0..59).
    InvalidMinutes(u32),
    /// Seconds value < 0.0 or ≥ 60.0 (valid range is 0.0..60.0).
    InvalidSeconds(f32),
    /// An unexpected character was encountered while parsing.
    InvalidCharacter(char, String),
    /// Invalid whitespace between DMS components.
    InvalidWhitespace(String),
    /// An improperly formatted numeric value.
    InvalidNumericFormat(String),
    /// An improper numeric value.
    InvalidNumericValue(f64),
    /// Unable to create a new Coordinate value.
    InvalidCoordinate,
    /// The URI scheme was not `geo:` as required by RFC 5870.
    #[cfg(feature = "urn")]
    InvalidUrnScheme,
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAngle(v, limit) => write!(
                f,
                "Invalid value `{v}` for angle; expecting `-{limit}..={limit}`."
            ),
            Error::InvalidLatitudeDegrees(v) => write!(
                f,
                "Invalid value `{v}` for latitude degrees; expecting `-90..=90`."
            ),
            Error::InvalidLongitudeDegrees(v) => write!(
                f,
                "Invalid value `{v}` for longitude degrees; expecting `-180..=180`."
            ),
            Error::InvalidMinutes(v) => {
                write!(f, "Invalid value `{v}` for minutes; expecting `0..60`.")
            }
            Error::InvalidSeconds(v) => {
                write!(f, "Invalid value `{v}` for seconds; expecting `0.0..60.0`.")
            }
            Error::InvalidCharacter(c, s) => {
                write!(f, "Invalid character `{c}` parsing value `\"{s}\"`.")
            }
            Error::InvalidWhitespace(s) => {
                write!(f, "Invalid whitespace parsing value `\"{s}\"`.")
            }
            Error::InvalidNumericFormat(s) => {
                write!(f, "Invalid numeric format parsing value `\"{s}\"`.")
            }
            Error::InvalidCoordinate => {
                write!(
                    f,
                    "Invalid `Latitude` and `Longitude` pair constructing `Coordinate`."
                )
            }
            #[cfg(feature = "url")]
            Error::InvalidUrnScheme => write!(f, "URI scheme must be `geo:`."),
            Error::InvalidNumericValue(v) => write!(f, "Invalid floating point value, `{v}`"),
        }
    }
}

impl std::error::Error for Error {}
