//!
//! This module provides an [`Altitude`] type, [`crate::lat!`] macro, and a [`Coordinate3d`]
//! structure which is a lat/long [`Coordinate`] with an altitude.
//!

use crate::{
    Coordinate, Error, Latitude, Longitude,
    fmt::{FormatOptions, Formatter},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Write},
    str::FromStr,
};
use uom::{
    fmt::DisplayStyle,
    si::{f64::Length, length},
};

#[cfg(feature = "geojson")]
use crate::Angle;
#[cfg(feature = "geojson")]
use crate::coord::{GEOJSON_COORDINATES_FIELD, GEOJSON_POINT_TYPE, GEOJSON_TYPE_FIELD};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

/// Quick creation of [`Altitude`] values.
///
/// * `alt!(10.0; cm)` create an altitude of 10 centimeters.
/// * `alt!(10.0; m)` create an altitude of 10 meters.
/// * `alt!(10.0; km)` create an altitude of 10 kilometers.
/// * `alt!(10.0)` create an altitude of 10 meters.
///
/// # Examples
///
/// ```rust
/// use lat_long::alt;
///
/// assert_eq!("10 m".to_string(), alt!(10.0; m).to_string());
/// ```
#[macro_export]
macro_rules! alt {
    ($value:expr; cm) => {
        $crate::alt::Altitude::centimeters($value)
    };
    ($value:expr; m) => {
        $crate::alt::Altitude::meters($value)
    };
    ($value:expr; km) => {
        $crate::alt::Altitude::kilometers($value)
    };
    ($value:expr) => {
        $crate::alt::Altitude::meters($value)
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// An altitude, in meters, above or below sea level.
#[allow(clippy::derive_ord_xor_partial_ord)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Altitude(Length);

/// A three dimensional geographic coordinate expressed as a (latitude, longitude, altitude) triple.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Altitude, Angle, Coordinate3d, Latitude, Longitude};
///
/// let lat = Latitude::try_from(47.6204).unwrap();
/// let lon = Longitude::try_from(-122.3491).unwrap();
/// let height = Altitude::meters(226.0);
/// let top_of_seattle_space_needle = Coordinate3d::new_from(lat, lon, height);
///
/// println!("{top_of_seattle_space_needle}");   // decimal degrees
/// println!("{top_of_seattle_space_needle:#}"); // degrees–minutes–seconds
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Coordinate3d {
    point: Coordinate,
    altitude: Altitude,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Altitude
// ------------------------------------------------------------------------------------------------

impl Display for Altitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.value() {
                0.001..0.01 => self
                    .0
                    .into_format_args(length::millimeter, DisplayStyle::Description)
                    .fmt(f),
                0.01..1.0 => self
                    .0
                    .into_format_args(length::centimeter, DisplayStyle::Description)
                    .fmt(f),
                1_000.0.. => self
                    .0
                    .into_format_args(length::kilometer, DisplayStyle::Description)
                    .fmt(f),
                _ => self
                    .0
                    .into_format_args(length::meter, DisplayStyle::Description)
                    .fmt(f),
            }
        } else {
            self.0
                .into_format_args(length::meter, DisplayStyle::Abbreviation)
                .fmt(f)
        }
    }
}

impl FromStr for Altitude {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length_float =
            f64::from_str(s).map_err(|_| Error::InvalidNumericFormat(s.to_string()))?;
        Self::try_from(length_float)
    }
}

impl TryFrom<Length> for Altitude {
    type Error = Error;

    fn try_from(value: Length) -> Result<Self, Self::Error> {
        Self::try_from(value.value)
    }
}

impl TryFrom<f64> for Altitude {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && !value.is_nan() {
            Ok(Self::meters(value))
        } else {
            Err(Error::InvalidNumericValue(value))
        }
    }
}

impl Eq for Altitude {}

impl Ord for Altitude {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.value.total_cmp(&other.0.value)
    }
}

impl From<Altitude> for Length {
    fn from(value: Altitude) -> Self {
        value.0
    }
}

impl From<Altitude> for f64 {
    fn from(value: Altitude) -> Self {
        value.0.value
    }
}

impl AsRef<Length> for Altitude {
    fn as_ref(&self) -> &Length {
        &self.0
    }
}

impl Altitude {
    /// Construct a new altitude value for sea level, i.e. `0 m`.
    pub fn sea_level() -> Self {
        Self::meters(0.0)
    }

    /// Construct an altitude in centimeters.
    pub fn centimeters(value: f64) -> Self {
        assert!(
            value.is_finite() && !value.is_nan(),
            "Invalid floating point value, `{value}`"
        );
        Self(Length::new::<length::centimeter>(value))
    }

    /// Construct an altitude in meters.
    pub fn meters(value: f64) -> Self {
        assert!(
            value.is_finite() && !value.is_nan(),
            "Invalid floating point value, `{value}`"
        );
        Self(Length::new::<length::meter>(value))
    }

    /// Construct an altitude in kilometers.
    pub fn kilometers(value: f64) -> Self {
        assert!(
            value.is_finite() && !value.is_nan(),
            "Invalid floating point value, `{value}`"
        );
        Self(Length::new::<length::kilometer>(value))
    }

    pub fn value(&self) -> f64 {
        self.0.value
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Coordinate3d
// ------------------------------------------------------------------------------------------------

impl Display for Coordinate3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = if f.alternate() {
            FormatOptions::dms()
        } else {
            FormatOptions::decimal()
        };
        self.format(f, &format)
    }
}

impl Formatter for Coordinate3d {
    fn format<W: Write>(&self, f: &mut W, options: &FormatOptions) -> std::fmt::Result {
        self.point.format(f, options)?;
        write!(f, ", {}", self.altitude)
    }
}

impl Coordinate3d {
    /// Construct a new 3d coordinate from a 2d point and an altitude.
    #[must_use]
    pub const fn new(point: Coordinate, altitude: Altitude) -> Self {
        Self { point, altitude }
    }

    /// Construct a new 3d coordinate from a 2d point, expressed as latitude and longitude
    /// values, and an altitude.
    #[must_use]
    pub const fn new_from(lat: Latitude, long: Longitude, altitude: Altitude) -> Self {
        Self::new(Coordinate::new(lat, long), altitude)
    }

    /// Return a new `Coordinate3d` with the point component replaced.
    #[must_use]
    pub const fn with_point(mut self, point: Coordinate) -> Self {
        self.point = point;
        self
    }

    /// Return a new `Coordinate3d` with the point component replaced by a new 2d coordinate.
    #[must_use]
    pub const fn with_new_point(mut self, lat: Latitude, long: Longitude) -> Self {
        self.point = Coordinate::new(lat, long);
        self
    }

    /// Return a new `Coordinate3d` with the altitude component replaced.
    #[must_use]
    pub const fn with_altitude(mut self, altitude: Altitude) -> Self {
        self.altitude = altitude;
        self
    }

    /// Returns the 2d coordinate component of this 3d coordinate.
    #[must_use]
    pub const fn point(&self) -> Coordinate {
        self.point
    }

    /// Returns the altitude component of this 3d coordinate.
    #[must_use]
    pub const fn altitude(&self) -> Altitude {
        self.altitude
    }

    /// Returns `true` if this coordinate lies on the equator.
    #[must_use]
    pub fn is_on_equator(&self) -> bool {
        self.point.latitude().is_on_equator()
    }

    /// Returns `true` if this coordinate is in the northern hemisphere.
    #[must_use]
    pub fn is_northern(&self) -> bool {
        self.point.latitude().is_northern()
    }

    /// Returns `true` if this coordinate is in the southern hemisphere.
    #[must_use]
    pub fn is_southern(&self) -> bool {
        self.point.latitude().is_southern()
    }

    /// Returns `true` if this coordinate lies on the international reference meridian.
    #[must_use]
    pub fn is_on_international_reference_meridian(&self) -> bool {
        self.point
            .longitude()
            .is_on_international_reference_meridian()
    }

    /// Returns `true` if this coordinate is in the western hemisphere.
    #[must_use]
    pub fn is_western(&self) -> bool {
        self.point.longitude().is_western()
    }

    /// Returns `true` if this coordinate is in the eastern hemisphere.
    #[must_use]
    pub fn is_eastern(&self) -> bool {
        self.point.longitude().is_eastern()
    }

    /// Returns `true` if this coordinate lies on the equator.
    #[must_use]
    pub fn is_at_sea_level(&self) -> bool {
        self.altitude.value() == 0.0
    }
}

#[cfg(feature = "geojson")]
impl From<Coordinate3d> for serde_json::Value {
    /// See [The GeoJSON Format](https://geojson.org/).
    fn from(coord: Coordinate3d) -> Self {
        serde_json::json!({
            GEOJSON_TYPE_FIELD: GEOJSON_POINT_TYPE,
            GEOJSON_COORDINATES_FIELD: [
                coord.point().latitude().as_float().0,
                coord.point().longitude().as_float().0,
                coord.altitude().value()
            ]
        })
    }
}

#[cfg(feature = "geojson")]
impl TryFrom<serde_json::Value> for Coordinate3d {
    type Error = crate::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value[GEOJSON_TYPE_FIELD] != GEOJSON_POINT_TYPE {
            return Err(crate::Error::InvalidCoordinate);
        }
        let coords = value[GEOJSON_COORDINATES_FIELD]
            .as_array()
            .ok_or(crate::Error::InvalidCoordinate)?;
        if coords.len() != 3 {
            return Err(crate::Error::InvalidCoordinate);
        }
        let lat_val: f64 = coords[0]
            .as_f64()
            .ok_or(crate::Error::InvalidNumericFormat(coords[0].to_string()))?;
        let lon_val: f64 = coords[1]
            .as_f64()
            .ok_or(crate::Error::InvalidNumericFormat(coords[1].to_string()))?;
        let alt_val: f64 = coords[2]
            .as_f64()
            .ok_or(crate::Error::InvalidNumericFormat(coords[2].to_string()))?;
        let lat = Latitude::try_from(lat_val)?;
        let lon = Longitude::try_from(lon_val)?;
        let alt = Altitude::try_from(alt_val)?;
        Ok(Coordinate3d::new_from(lat, lon, alt))
    }
}
