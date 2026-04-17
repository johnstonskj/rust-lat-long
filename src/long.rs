//! This module provides the [`Longitude`] type, [`crate::long!`] macro, and associated constants.

use crate::{
    Angle, Error,
    fmt::{FormatOptions, Formatter, formatter_impl},
    inner,
    parse::{self, Parsed, Value},
};
use core::{
    fmt::{Debug, Display, Write},
    str::FromStr,
};
use ordered_float::OrderedFloat;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

/// A geographic longitude value, constrained to **−180 ≤ degrees ≤ 180**.
///
/// Positive values are east of the international reference meridian; negative
/// values are west.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Angle, Longitude};
///
/// let lon = Longitude::new(-73, 56, 0.0).unwrap();
/// assert!(lon.is_western());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Longitude(OrderedFloat<f64>);

// ---------------------------------------------------------------------------
// Public Constants
// ---------------------------------------------------------------------------

/// IERS International Reference Meridian (IRM), or Prime Meridian, at 0° longitude.
pub const INTERNATIONAL_REFERENCE_MERIDIAN: Longitude = Longitude(inner::ZERO);

/// Antimeridian, the basis for the International Date Line (IDL), at 180° longitude.
pub const ANTI_MERIDIAN: Longitude = Longitude(OrderedFloat(LONGITUDE_LIMIT));

// ---------------------------------------------------------------------------
// Public Macros
// ---------------------------------------------------------------------------

#[macro_export]
macro_rules! long {
    (E $degrees:expr, $minutes:expr, $seconds:expr) => {
        long!($degrees.abs(), $minutes, $seconds).unwrap()
    };
    (W $degrees:expr, $minutes:expr, $seconds:expr) => {
        long!(-$degrees.abs(), $minutes, $seconds).unwrap()
    };
    ($degrees:expr, $minutes:expr, $seconds:expr) => {
        Longitude::new($degrees, $minutes, $seconds).unwrap()
    };
    (E $degrees:expr, $minutes:expr) => {
        long!($degrees.abs(), $minutes).unwrap()
    };
    (W $degrees:expr, $minutes:expr) => {
        long!(-$degrees.abs(), $minutes).unwrap()
    };
    ($degrees:expr, $minutes:expr) => {
        long!($degrees, $minutes, 0.0).unwrap()
    };
    (E $degrees:expr) => {
        long!($degrees.abs()).unwrap()
    };
    (W $degrees:expr) => {
        long!(-$degrees.abs()).unwrap()
    };
    ($degrees:expr) => {
        long!($degrees, 0, 0.0).unwrap()
    };
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------

const LONGITUDE_LIMIT: f64 = 180.0;

impl Default for Longitude {
    fn default() -> Self {
        INTERNATIONAL_REFERENCE_MERIDIAN
    }
}

impl TryFrom<f64> for Longitude {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_from(OrderedFloat(value))
    }
}

impl TryFrom<OrderedFloat<f64>> for Longitude {
    type Error = Error;

    fn try_from(value: OrderedFloat<f64>) -> Result<Self, Self::Error> {
        if value.is_infinite() || value.is_nan() {
            Err(Error::InvalidNumericValue(value.into()))
        } else if value.0 < -LONGITUDE_LIMIT || value.0 > LONGITUDE_LIMIT {
            return Err(Error::InvalidAngle(value.into_inner(), LONGITUDE_LIMIT));
        } else {
            Ok(Self(value))
        }
    }
}

impl From<Longitude> for OrderedFloat<f64> {
    fn from(value: Longitude) -> Self {
        value.0.into()
    }
}

impl From<Longitude> for f64 {
    fn from(value: Longitude) -> Self {
        value.0.into()
    }
}

impl FromStr for Longitude {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse::parse_str(s)? {
            Parsed::Angle(Value::Unknown(decimal)) => Self::try_from(decimal),
            Parsed::Angle(Value::Longitude(lon)) => Ok(lon),
            _ => Err(Error::InvalidAngle(0.0, 0.0)),
        }
    }
}

impl Display for Longitude {
    /// Formats the longitude as decimal degrees by default, or as
    /// degrees–minutes–seconds when the alternate flag (`{:#}`) is used.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let mut buf = String::new();
            self.format(&mut buf, &FormatOptions::dms_signed())?;
            f.write_str(&buf)
        } else {
            Display::fmt(&(self.0), f)
        }
    }
}

impl Formatter for Longitude {
    fn format<W: Write>(&self, f: &mut W, fmt: &FormatOptions) -> std::fmt::Result {
        let fmt = (*fmt).with_labels(('E', 'W'));
        formatter_impl(self.0, f, &fmt)
    }
}

impl Angle for Longitude {
    const MIN: Self = Self(OrderedFloat(-LONGITUDE_LIMIT));
    const MAX: Self = Self(OrderedFloat(LONGITUDE_LIMIT));

    fn new(degrees: i32, minutes: u32, seconds: f32) -> Result<Self, Error> {
        if degrees < Self::MIN.as_float().0 as i32 || degrees > Self::MAX.as_float().0 as i32 {
            return Err(Error::InvalidLongitudeDegrees(degrees));
        }
        let float = inner::from_degrees_minutes_seconds(degrees, minutes, seconds)?;
        Self::try_from(float).map_err(|_| Error::InvalidLongitudeDegrees(degrees))
    }

    fn as_float(&self) -> OrderedFloat<f64> {
        self.0
    }
}

impl Longitude {
    /// Returns `true` if this longitude is exactly on the IERS International Reference Meridian (IRM), or 0°.
    #[must_use]
    pub fn is_on_international_reference_meridian(&self) -> bool {
        self.is_zero()
    }

    /// Returns `true` if this longitude is in the western hemisphere (< 0°).
    #[must_use]
    pub fn is_western(&self) -> bool {
        self.is_nonzero_negative()
    }

    /// Returns `true` if this longitude is in the eastern hemisphere (> 0°).
    #[must_use]
    pub fn is_eastern(&self) -> bool {
        self.is_nonzero_positive()
    }
}
