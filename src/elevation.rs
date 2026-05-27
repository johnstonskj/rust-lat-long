//!
//! This module provides an [`Elevation`] type, [`crate::elv!`] macro, and a [`CoordinateWithElevation`]
//! structure which is a lat/long [`Coordinate`] with an associated elevation.
//!
//! The elevation of a geographic location is its height above or below a fixed reference point, most
//! commonly a reference geoid, a mathematical model of the Earth's sea level as an equipotential
//! gravitational surface (see Geodetic datum § Vertical datum).
//!
//! The reference point for the type [`Elevation`] is not defined, therefore any absolute value **must**
//! be calculated by conversion from convention in reference to some datum.
//!

use crate::{
    Coordinate, Error, Latitude, Longitude,
    fmt::{FormatOptions, Formatter},
};
use core::hash::Hash;
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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

/// Quick creation of [`Elevation`] values.
///
/// * `elv!(10.0; cm)` create an elevation of 10 centimeters.
/// * `elv!(10.0; m)` create an elevation of 10 meters.
/// * `elv!(10.0; km)` create an elevation of 10 kilometers.
/// * `elv!(10.0)` create an elevation of 10 meters.
///
/// # Examples
///
/// ```rust
/// use lat_long::elv;
///
/// assert_eq!("10 m".to_string(), elv!(10.0; m).to_string());
/// ```
#[macro_export]
macro_rules! elv {
    ($value:expr; cm) => {
        $crate::elevation::Elevation::centimeters($value)
    };
    ($value:expr; m) => {
        $crate::elevation::Elevation::meters($value)
    };
    ($value:expr; km) => {
        $crate::elevation::Elevation::kilometers($value)
    };
    ($value:expr) => {
        $crate::elevation::Elevation::meters($value)
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An elevation, in meters, above or below an undefined reference level.
///
#[allow(clippy::derive_ord_xor_partial_ord)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Elevation(Length);

///
/// A three dimensional geographic coordinate expressed as a (latitude, longitude, elevation) triple.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Elevation, Angle, CoordinateWithElevation, Latitude, Longitude};
///
/// let lat = Latitude::try_from(47.6204).unwrap();
/// let lon = Longitude::try_from(-122.3491).unwrap();
/// let height = Elevation::meters(226.0);
/// let top_of_seattle_space_needle = CoordinateWithElevation::new_from(lat, lon, height);
///
/// println!("{top_of_seattle_space_needle}");   // decimal degrees
/// println!("{top_of_seattle_space_needle:#}"); // degrees–minutes–seconds
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CoordinateWithElevation {
    point: Coordinate,
    elevation: Elevation,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Elevation
// ------------------------------------------------------------------------------------------------

const ELEVATION_ZERO: f64 = 0.0;

impl Display for Elevation {
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

impl FromStr for Elevation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length_float =
            f64::from_str(s).map_err(|_| Error::InvalidNumericFormat(s.to_string()))?;
        Self::try_from(length_float)
    }
}

impl TryFrom<Length> for Elevation {
    type Error = Error;

    fn try_from(value: Length) -> Result<Self, Self::Error> {
        Self::try_from(value.value)
    }
}

impl TryFrom<f64> for Elevation {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && !value.is_nan() {
            Ok(Self::meters(value))
        } else {
            Err(Error::InvalidNumericValue(value))
        }
    }
}

impl Eq for Elevation {}

impl Hash for Elevation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.value.to_bits().hash(state);
    }
}

impl Ord for Elevation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.value.total_cmp(&other.0.value)
    }
}

impl From<Elevation> for Length {
    fn from(value: Elevation) -> Self {
        value.0
    }
}

impl From<Elevation> for f64 {
    fn from(value: Elevation) -> Self {
        value.0.value
    }
}

impl AsRef<Length> for Elevation {
    fn as_ref(&self) -> &Length {
        &self.0
    }
}

impl Elevation {
    ///
    /// Zero is ambiguous as an elevation, since it could represent either sea level, or the ground level
    /// at the location of interest, or a Geodetic [vertical datum](https://en.wikipedia.org/wiki/Vertical_datum).
    ///
    pub fn zero() -> Self {
        Self(Length::new::<length::meter>(ELEVATION_ZERO))
    }

    ///
    /// Construct an elevation in centimeters.
    ///
    pub fn centimeters(value: f64) -> Self {
        assert!(
            value.is_finite() && !value.is_nan(),
            "Invalid floating point value, `{value}`"
        );
        Self(Length::new::<length::centimeter>(value))
    }

    ///
    /// Construct an elevation in meters.
    ///
    pub fn meters(value: f64) -> Self {
        assert!(
            value.is_finite() && !value.is_nan(),
            "Invalid floating point value, `{value}`"
        );
        Self(Length::new::<length::meter>(value))
    }

    ///
    /// Construct an elevation in kilometers.
    ///
    pub fn kilometers(value: f64) -> Self {
        assert!(
            value.is_finite() && !value.is_nan(),
            "Invalid floating point value, `{value}`"
        );
        Self(Length::new::<length::kilometer>(value))
    }

    ///
    /// Returns the elevation value in meters as an `f64`.
    ///
    pub fn value(&self) -> f64 {
        self.0.value
    }

    ///
    /// Returns `true` if this elevation is exactly zero.
    ///
    pub fn is_zero(&self) -> bool {
        self.0.value == ELEVATION_ZERO
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ CoordinateWithElevation
// ------------------------------------------------------------------------------------------------

impl Display for CoordinateWithElevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = if f.alternate() {
            FormatOptions::dms()
        } else {
            FormatOptions::decimal()
        };
        self.format(f, &format)
    }
}

impl Formatter for CoordinateWithElevation {
    fn format<W: Write>(&self, f: &mut W, options: &FormatOptions) -> std::fmt::Result {
        self.point.format(f, options)?;
        write!(f, ", {}", self.elevation)
    }
}

impl FromStr for CoordinateWithElevation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((coordinate, elevation)) = s.rsplit_once(',') {
            Ok(Self::new(
                Coordinate::from_str(coordinate.trim())?,
                Elevation::from_str(elevation.trim())?,
            ))
        } else {
            Err(Error::InvalidCoordinate)
        }
    }
}

impl CoordinateWithElevation {
    ///
    /// Construct a new 3d coordinate from a 2d point and an elevation.
    ///
    #[must_use]
    pub const fn new(point: Coordinate, elevation: Elevation) -> Self {
        Self { point, elevation }
    }

    /// Construct a new 3d coordinate from a 2d point, expressed as latitude and longitude
    /// values, and an elevation.
    #[must_use]
    pub const fn new_from(lat: Latitude, long: Longitude, elevation: Elevation) -> Self {
        Self::new(Coordinate::new(lat, long), elevation)
    }

    /// Return a new 3d coordinate with the point component replaced.
    #[must_use]
    pub const fn with_point(mut self, point: Coordinate) -> Self {
        self.point = point;
        self
    }

    /// Return a new 3d coordinate with the point component replaced by a new 2d coordinate.
    #[must_use]
    pub const fn with_new_point(mut self, lat: Latitude, long: Longitude) -> Self {
        self.point = Coordinate::new(lat, long);
        self
    }

    /// Return a new `CoordinateWithElevation` with the elevation component replaced.
    #[must_use]
    pub const fn with_elevation(mut self, elevation: Elevation) -> Self {
        self.elevation = elevation;
        self
    }

    /// Returns the 2d coordinate component of this 3d coordinate.
    #[must_use]
    pub const fn point(&self) -> Coordinate {
        self.point
    }

    /// Returns the elevation component of this 3d coordinate.
    #[must_use]
    pub const fn elevation(&self) -> Elevation {
        self.elevation
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
    pub fn is_zero_elevation(&self) -> bool {
        self.elevation.is_zero()
    }
}

#[cfg(feature = "geojson")]
impl From<CoordinateWithElevation> for serde_json::Value {
    /// See [The GeoJSON Format](https://geojson.org/).
    fn from(coord: CoordinateWithElevation) -> Self {
        serde_json::json!({
            GEOJSON_TYPE_FIELD: GEOJSON_POINT_TYPE,
            GEOJSON_COORDINATES_FIELD: [
                coord.point().latitude().as_float().0,
                coord.point().longitude().as_float().0,
                coord.elevation().value()
            ]
        })
    }
}

#[cfg(feature = "geojson")]
impl TryFrom<serde_json::Value> for CoordinateWithElevation {
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
        let alt = Elevation::try_from(alt_val)?;
        Ok(CoordinateWithElevation::new_from(lat, lon, alt))
    }
}
