//! This module provides the [`Latitude`] type, [`crate::lat!`] macro, and associated constants.
//!

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

/// A geographic latitude value, constrained to **−90 ≤ degrees ≤ 90**.
///
/// Positive values are north of the equator; negative values are south.
///
/// # Construction
///
/// Use [`Latitude::new`] to construct from degrees, minutes, and seconds, or
/// [`TryFrom<inner::Float>`] if you already have a decimal-degree value.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Angle, Latitude};
///
/// let lat = Latitude::new(45, 30, 0.0).unwrap();
/// assert!(lat.is_northern());
///
/// let equator = Latitude::new(0, 0, 0.0).unwrap();
/// assert!(equator.is_on_equator());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Latitude(OrderedFloat<f64>);

// ---------------------------------------------------------------------------
// Public Constants
// ---------------------------------------------------------------------------

/// The geographic North Pole, at 90° N latitude.
pub const NORTH_POLE: Latitude = Latitude(OrderedFloat(LATITUDE_LIMIT));

/// The Arctic Circle, approximately 66.5° N latitude.
///
/// Latitudes at or above this value experience at least one full day of
/// continuous daylight or darkness per year.
pub const ARCTIC_CIRCLE: Latitude = Latitude(OrderedFloat(66.5));

/// The Tropic of Cancer, approximately 23.5° N latitude.
///
/// The northernmost latitude at which the sun can appear directly overhead
/// at solar noon (at the June solstice).
pub const TROPIC_OF_CANCER: Latitude = Latitude(OrderedFloat(23.5));

/// The equator, at 0° latitude.
///
/// The circle of latitude equidistant from both poles, dividing the globe
/// into the northern and southern hemispheres.
pub const EQUATOR: Latitude = Latitude(inner::ZERO);

/// The Tropic of Capricorn, approximately 23.5° S latitude.
///
/// The southernmost latitude at which the sun can appear directly overhead
/// at solar noon (at the December solstice).
pub const TROPIC_OF_CAPRICORN: Latitude = Latitude(OrderedFloat(-23.5));

/// The Antarctic Circle, approximately 66.5° S latitude.
///
/// Latitudes at or below this value experience at least one full day of
/// continuous daylight or darkness per year.
pub const ANTARCTIC_CIRCLE: Latitude = Latitude(OrderedFloat(-66.5));

/// The geographic South Pole, at 90° S latitude.
pub const SOUTH_POLE: Latitude = Latitude(OrderedFloat(-LATITUDE_LIMIT));

// ---------------------------------------------------------------------------
// Public Macros
// ---------------------------------------------------------------------------

#[macro_export]
macro_rules! lat {
    (N $degrees:expr, $minutes:expr, $seconds:expr) => {
        lat!($degrees.abs(), $minutes, $seconds).unwrap()
    };
    (S $degrees:expr, $minutes:expr, $seconds:expr) => {
        lat!(-$degrees.abs(), $minutes, $seconds).unwrap()
    };
    ($degrees:expr, $minutes:expr, $seconds:expr) => {
        Latitude::new($degrees, $minutes, $seconds).unwrap()
    };
    (N $degrees:expr, $minutes:expr) => {
        lat!($degrees.abs(), $minutes).unwrap()
    };
    (S $degrees:expr, $minutes:expr) => {
        lat!(-$degrees.abs(), $minutes).unwrap()
    };
    ($degrees:expr, $minutes:expr) => {
        lat!($degrees, $minutes, 0.0).unwrap()
    };
    (N $degrees:expr) => {
        lat!($degrees.abs()).unwrap()
    };
    (S $degrees:expr) => {
        lat!(-$degrees.abs()).unwrap()
    };
    ($degrees:expr) => {
        lat!($degrees, 0, 0.0).unwrap()
    };
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------

const LATITUDE_LIMIT: f64 = 90.0;

impl Default for Latitude {
    fn default() -> Self {
        EQUATOR
    }
}

impl TryFrom<f64> for Latitude {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_from(OrderedFloat(value))
    }
}

impl TryFrom<OrderedFloat<f64>> for Latitude {
    type Error = Error;

    fn try_from(value: OrderedFloat<f64>) -> Result<Self, Self::Error> {
        if value.is_infinite() || value.is_nan() {
            Err(Error::InvalidNumericValue(value.into()))
        } else if value.0 < -LATITUDE_LIMIT || value.0 > LATITUDE_LIMIT {
            Err(Error::InvalidAngle(value.into_inner(), LATITUDE_LIMIT))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<Latitude> for OrderedFloat<f64> {
    fn from(value: Latitude) -> Self {
        value.0.into()
    }
}

impl From<Latitude> for f64 {
    fn from(value: Latitude) -> Self {
        value.0.into()
    }
}

impl FromStr for Latitude {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse::parse_str(s)? {
            Parsed::Angle(Value::Unknown(decimal)) => Self::try_from(decimal),
            Parsed::Angle(Value::Latitude(lat)) => Ok(lat),
            _ => Err(Error::InvalidAngle(0.0, 0.0)),
        }
    }
}

impl Display for Latitude {
    /// Formats the latitude as decimal degrees by default, or as
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

impl Formatter for Latitude {
    fn format<W: Write>(&self, f: &mut W, fmt: &FormatOptions) -> std::fmt::Result {
        let fmt = (*fmt).with_labels(('N', 'S'));
        formatter_impl(self.0, f, &fmt)
    }
}

impl Angle for Latitude {
    const MIN: Self = Self(OrderedFloat(-LATITUDE_LIMIT));
    const MAX: Self = Self(OrderedFloat(LATITUDE_LIMIT));

    fn new(degrees: i32, minutes: u32, seconds: f32) -> Result<Self, Error> {
        if degrees < Self::MIN.as_float().0 as i32 || degrees > Self::MAX.as_float().0 as i32 {
            return Err(Error::InvalidLatitudeDegrees(degrees));
        }
        // Delegate to inner helper; it verifies minutes/seconds.
        // The only remaining failure path from try_from is if the decimal
        // representation exceeds the limit (e.g. 90°0′0.000001″) — still
        // report as InvalidLatitudeDegrees.
        let float = inner::from_degrees_minutes_seconds(degrees, minutes, seconds)?;
        Self::try_from(float).map_err(|_| Error::InvalidLatitudeDegrees(degrees))
    }

    fn as_float(&self) -> OrderedFloat<f64> {
        self.0
    }
}

impl Latitude {
    /// Returns `true` if this latitude is exactly on the equator (0°).
    #[must_use]
    pub fn is_on_equator(&self) -> bool {
        self.is_zero()
    }

    /// Returns `true` if this latitude is in the northern hemisphere (> 0°).
    #[must_use]
    pub fn is_northern(&self) -> bool {
        self.is_nonzero_positive()
    }

    /// Returns `true` if this latitude is in the southern hemisphere (< 0°).
    #[must_use]
    pub fn is_southern(&self) -> bool {
        self.is_nonzero_negative()
    }

    /// Returns `true` if this latitude is within the Arctic region (≥ [`ARCTIC_CIRCLE`], i.e. ≥ 66.5° N).
    #[must_use]
    pub fn is_arctic(&self) -> bool {
        *self >= ARCTIC_CIRCLE
    }

    /// Returns `true` if this latitude is within the Antarctic region (≤ [`ANTARCTIC_CIRCLE`], i.e. ≤ 66.5° S).
    #[must_use]
    pub fn is_antarctic(&self) -> bool {
        *self <= ANTARCTIC_CIRCLE
    }

    /// Returns `true` if this latitude is at or north of the [`TROPIC_OF_CANCER`] (≥ 23.5° N).
    ///
    /// Together with [`is_tropic_of_capricorn`](Self::is_tropic_of_capricorn) this is used to
    /// identify locations within the tropical band.
    #[must_use]
    pub fn is_tropic_of_cancer(&self) -> bool {
        *self >= TROPIC_OF_CANCER
    }

    /// Returns `true` if this latitude is at or south of the [`TROPIC_OF_CAPRICORN`] (≤ 23.5° S).
    #[must_use]
    pub fn is_tropic_of_capricorn(&self) -> bool {
        *self <= TROPIC_OF_CAPRICORN
    }

    /// Returns `true` if this latitude lies within the tropical band (between the
    /// [`TROPIC_OF_CANCER`] and [`TROPIC_OF_CAPRICORN`], i.e. within ±23.5°).
    ///
    /// Note: this returns `true` for latitudes *outside* the tropical band that
    /// are ≥ [`TROPIC_OF_CANCER`] in the north or ≤ [`TROPIC_OF_CAPRICORN`] in
    /// the south — see individual methods for precise semantics.
    #[must_use]
    pub fn is_tropical(&self) -> bool {
        self.is_tropic_of_cancer() || self.is_tropic_of_capricorn()
    }

    /// Returns `true` if this latitude is within either polar region
    /// (at or beyond [`ARCTIC_CIRCLE`] north or [`ANTARCTIC_CIRCLE`] south).
    #[must_use]
    pub fn is_polar(&self) -> bool {
        self.is_arctic() || self.is_antarctic()
    }
}
