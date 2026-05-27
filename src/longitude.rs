//! This module provides the [`Longitude`] type, [`crate::long!`] macro, and associated constants.
//!
//! Longitude is a geographic coordinate that specifies the east-west position of a point on the
//! surface of the Earth. It is an angular measurement, usually expressed in degrees and denoted
//! by the Greek letter lambda (λ). Meridians are imaginary semicircular lines running from pole
//! to pole that connect points with the same longitude. The prime meridian defines 0° longitude;
//! by convention the International Reference Meridian for the Earth passes near the Royal
//! Observatory in Greenwich, south-east London on the island of Great Britain. Positive longitudes
//! are east of the prime meridian, and negative ones are west.
//!
//! The longitude denoted by the type [`Longitude`] is not strictly a *Geodetic Longitude* in that it
//! is not defined in relation to some reference geodetic datum but some abstract center of mass.
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

///
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
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Longitude(OrderedFloat<f64>);

// ---------------------------------------------------------------------------
// Public Constants
// ---------------------------------------------------------------------------

///
/// IERS International Reference Meridian (IRM), or Prime Meridian, at 0° longitude.
///
pub const INTERNATIONAL_REFERENCE_MERIDIAN: Longitude = Longitude(inner::ZERO);

///
/// Antimeridian, the basis for the International Date Line (IDL), at 180° longitude.
///
pub const ANTI_MERIDIAN: Longitude = Longitude(OrderedFloat(LONGITUDE_LIMIT));

// ---------------------------------------------------------------------------
// Public Macros
// ---------------------------------------------------------------------------

///
/// Ergonomic constructor for [`Longitude`] values.
///
/// All forms `.unwrap()` internally — they are intended for compile-time-known
/// constants and tests where invalid input is a bug. Use [`Longitude::new`]
/// when you need to handle validation errors.
///
/// | Form                                | Example                   | Meaning              |
/// |-------------------------------------|---------------------------|----------------------|
/// | `long!(d)`                          | `long!(2)`                | 2° E                 |
/// | `long!(d, m)`                       | `long!(2, 21)`            | 2° 21′ E             |
/// | `long!(d, m, s)`                    | `long!(2, 21, 8.0)`       | 2° 21′ 8″ E          |
/// | `long!(E d, …)` / `long!(W d, …)`   | `long!(W 73, 59, 8.4)`    | explicit hemisphere  |
///
/// The `E`/`W` prefix forms take the absolute value of the degree argument
/// and apply the sign matching the direction.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Angle, Longitude, long};
///
/// let lon = long!(2, 21, 8.0);
/// assert!(lon.is_eastern());
/// assert_eq!(lon.degrees(), 2);
/// ```
///
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
            Err(Error::InvalidAngle(value.into_inner(), LONGITUDE_LIMIT))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<Longitude> for OrderedFloat<f64> {
    fn from(value: Longitude) -> Self {
        value.0
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
    ///
    /// Formats the longitude as decimal degrees by default, or as
    /// degrees–minutes–seconds when the alternate flag (`{:#}`) is used.
    ///
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
    ///
    /// Returns `true` if this longitude is exactly on the IERS International Reference Meridian (IRM), or 0°.
    ///
    #[must_use]
    pub fn is_on_international_reference_meridian(&self) -> bool {
        self.is_zero()
    }

    ///
    /// Returns `true` if this longitude is in the western hemisphere (< 0°).
    ///
    #[must_use]
    pub fn is_western(&self) -> bool {
        self.is_nonzero_negative()
    }

    ///
    /// Returns `true` if this longitude is in the eastern hemisphere (> 0°).
    ///
    #[must_use]
    pub fn is_eastern(&self) -> bool {
        self.is_nonzero_positive()
    }
    ///
    /// Return the [UTM longitude zone](https://en.wikipedia.org/wiki/Universal_Transverse_Mercator_coordinate_system#UTM_zone)
    /// number (1–60) covering this longitude.
    ///
    /// Zones are 6° wide and are numbered starting at the antimeridian
    /// (180° W = zone 1) and increasing eastward.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Longitude};
    ///
    /// // Seattle ~ -122.3° falls in zone 10.
    /// let lon = Longitude::try_from(-122.3).unwrap();
    /// assert_eq!(lon.utm_zone(), 10);
    /// ```
    ///
    #[must_use]
    pub fn utm_zone(&self) -> u8 {
        // UTM zones are 6° wide, numbered 1–60 starting at 180°W.
        // The formula below maps the range (−180, 180] to (0, 60], with 0 and 60 both representing the same zone.
        let zone = ((self.0 + LONGITUDE_LIMIT) / 6.0).ceil() as u8;
        if zone == 0 { 60 } else { zone }
    }
}
