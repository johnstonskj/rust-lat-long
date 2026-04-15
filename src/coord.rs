//! This module provides the [`Coordinate`] type, [`crate::coord!`] macro, and associated constants.

use crate::{
    Error, Latitude, Longitude,
    fmt::{FormatKind, FormatOptions, Formatter},
    parse::{self, Parsed},
};
use core::{
    fmt::{Debug, Display, Write},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "geojson")]
use crate::Angle;

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

/// A geographic coordinate expressed as a (latitude, longitude) pair.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Angle, Coordinate, Latitude, Longitude};
///
/// let lat = Latitude::new(51, 30, 26.0).unwrap();
/// let lon = Longitude::new(0, 7, 39.0).unwrap();
/// let london = Coordinate::new(lat, lon);
///
/// println!("{london}");   // decimal degrees
/// println!("{london:#}"); // degrees–minutes–seconds
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Coordinate {
    lat: Latitude,   // φ
    long: Longitude, // λ
}

// ---------------------------------------------------------------------------
// Public Constants
// ---------------------------------------------------------------------------

/// The URI scheme used by [`Coordinate::to_url_string`] to format a `geo:` URI.
///
/// Defined by [RFC 5870](https://www.rfc-editor.org/rfc/rfc5870).
pub const GEO_URL_SCHEME: &str = "geo";

#[cfg(feature = "geojson")]
pub const GEOJSON_TYPE_FIELD: &str = "type";
#[cfg(feature = "geojson")]
pub const GEOJSON_COORDINATES_FIELD: &str = "coordinates";
#[cfg(feature = "geojson")]
pub const GEOJSON_POINT_TYPE: &str = "Point";

// ---------------------------------------------------------------------------
// Public Macros
// ---------------------------------------------------------------------------

#[macro_export]
macro_rules! coord {
    ($lat:expr ; $lon:expr) => {
        Coordinate::new($lat, $lon)
    };
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------

impl From<(Latitude, Longitude)> for Coordinate {
    fn from(value: (Latitude, Longitude)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Coordinate> for (Latitude, Longitude) {
    fn from(value: Coordinate) -> Self {
        (value.lat, value.long)
    }
}

impl From<Latitude> for Coordinate {
    fn from(value: Latitude) -> Self {
        Self::new(value, Longitude::default())
    }
}

impl From<Longitude> for Coordinate {
    fn from(value: Longitude) -> Self {
        Self::new(Latitude::default(), value)
    }
}

impl FromStr for Coordinate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse::parse_str(s)? {
            Parsed::Coordinate(coord) => Ok(coord),
            _ => Err(Error::InvalidAngle(0.0, 0.0)),
        }
    }
}

impl Display for Coordinate {
    /// Formats the coordinate as `"latitude, longitude"`.
    ///
    /// Uses decimal degrees by default; the alternate flag (`{:#}`) switches
    /// both components to degrees–minutes–seconds.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = if f.alternate() {
            FormatOptions::dms()
        } else {
            FormatOptions::decimal()
        };
        self.format(f, &format)
    }
}

impl Formatter for Coordinate {
    fn format<W: Write>(&self, f: &mut W, fmt: &FormatOptions) -> std::fmt::Result {
        let kind = fmt.kind();
        self.lat.format(f, fmt)?;
        write!(f, ",{}", if kind == FormatKind::DmsBare { "" } else { " " })?;
        self.long.format(f, fmt)
    }
}

impl Coordinate {
    /// Construct a new `Coordinate` from a validated [`Latitude`] and [`Longitude`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Coordinate, Latitude, Longitude};
    ///
    /// let lat = Latitude::new(48, 51, 30.0).unwrap();
    /// let lon = Longitude::new(2, 21, 8.0).unwrap();
    /// let paris = Coordinate::new(lat, lon);
    /// assert!(paris.is_northern());
    /// assert!(paris.is_eastern());
    /// ```
    pub const fn new(lat: Latitude, long: Longitude) -> Self {
        Self { lat, long }
    }

    /// Return a new `Coordinate` with the latitude component replaced.
    #[must_use]
    pub const fn with_latitude(mut self, lat: Latitude) -> Self {
        self.lat = lat;
        self
    }

    /// Return a new `Coordinate` with the longitude component replaced.
    #[must_use]
    pub const fn with_longitude(mut self, long: Longitude) -> Self {
        self.long = long;
        self
    }

    /// Returns the latitude component of this coordinate.
    #[must_use]
    pub const fn latitude(&self) -> Latitude {
        self.lat
    }

    /// Returns the latitude component of this coordinate.
    #[must_use]
    pub const fn φ(&self) -> Latitude {
        self.lat
    }

    /// Returns the longitude component of this coordinate.
    #[must_use]
    pub const fn longitude(&self) -> Longitude {
        self.long
    }

    /// Returns the longitude component of this coordinate.
    #[must_use]
    pub const fn λ(&self) -> Longitude {
        self.long
    }

    /// Returns `true` if this coordinate lies on the equator.
    #[must_use]
    pub fn is_on_equator(&self) -> bool {
        self.lat.is_on_equator()
    }

    /// Returns `true` if this coordinate is in the northern hemisphere.
    #[must_use]
    pub fn is_northern(&self) -> bool {
        self.lat.is_northern()
    }

    /// Returns `true` if this coordinate is in the southern hemisphere.
    #[must_use]
    pub fn is_southern(&self) -> bool {
        self.lat.is_southern()
    }

    /// Returns `true` if this coordinate lies on the international reference meridian.
    #[must_use]
    pub fn is_on_international_reference_meridian(&self) -> bool {
        self.long.is_on_international_reference_meridian()
    }

    /// Returns `true` if this coordinate is in the western hemisphere.
    #[must_use]
    pub fn is_western(&self) -> bool {
        self.long.is_western()
    }

    /// Returns `true` if this coordinate is in the eastern hemisphere.
    #[must_use]
    pub fn is_eastern(&self) -> bool {
        self.long.is_eastern()
    }

    /// Format this coordinate as a `geo:` URI string.
    ///
    /// The format is `geo:<lat>,<lon>` using decimal degrees with 8 places of
    /// precision, as per [RFC 5870](https://www.rfc-editor.org/rfc/rfc5870).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Coordinate, Latitude, Longitude};
    ///
    /// let lat = Latitude::new(48, 51, 30.0).unwrap();
    /// let lon = Longitude::new(2, 21, 8.0).unwrap();
    /// let paris = Coordinate::new(lat, lon);
    /// assert!(paris.to_url_string().starts_with("geo:"));
    /// ```
    #[must_use]
    pub fn to_url_string(&self) -> String {
        format!(
            "{}:{},{}",
            GEO_URL_SCHEME,
            self.lat.to_formatted_string(&FormatOptions::decimal()),
            self.long.to_formatted_string(&FormatOptions::decimal())
        )
    }

    /// Format this coordinate as a microformat string.
    ///
    /// This follows the microformat standard for representing coordinates specified
    /// in [mf-geo](https://microformats.org/wiki/geo) and referenced by
    /// [hCard](https://microformats.org/wiki/hcard) and
    /// [hCalendar](https://microformats.org/wiki/hcalendar).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Coordinate, Latitude, Longitude};
    ///
    /// let lat = Latitude::new(48, 51, 30.0).unwrap();
    /// let lon = Longitude::new(2, 21, 8.0).unwrap();
    /// let paris = Coordinate::new(lat, lon);
    /// assert!(paris.to_microformat_string().contains("class=\"latitude\""));
    /// assert!(paris.to_microformat_string().contains("class=\"longitude\""));
    /// ```
    #[must_use]
    pub fn to_microformat_string(&self) -> String {
        format!(
            "<span class=\"latitude\">{}</span>; <span class=\"longitude\">{}</span>",
            self.lat.to_formatted_string(&FormatOptions::decimal()),
            self.long.to_formatted_string(&FormatOptions::decimal())
        )
    }
}

#[cfg(feature = "urn")]
impl From<Coordinate> for url::Url {
    fn from(coord: Coordinate) -> Self {
        Self::parse(&coord.to_url_string()).unwrap()
    }
}

#[cfg(feature = "urn")]
impl TryFrom<url::Url> for Coordinate {
    type Error = crate::Error;

    fn try_from(url: url::Url) -> Result<Self, Self::Error> {
        if url.scheme() != GEO_URL_SCHEME {
            return Err(crate::Error::InvalidUrnScheme);
        }
        let path = url.path();
        let parts: Vec<&str> = path.split(',').collect();
        if parts.len() != 2 {
            return Err(crate::Error::InvalidCoordinate);
        }
        let lat_val: f64 = parts[0]
            .parse()
            .map_err(|_| crate::Error::InvalidCoordinate)?;
        let lon_val: f64 = parts[1]
            .parse()
            .map_err(|_| crate::Error::InvalidCoordinate)?;
        let lat = Latitude::try_from(lat_val).map_err(|_| crate::Error::InvalidCoordinate)?;
        let lon = Longitude::try_from(lon_val).map_err(|_| crate::Error::InvalidCoordinate)?;
        Ok(Coordinate::new(lat, lon))
    }
}

#[cfg(feature = "geojson")]
impl From<Coordinate> for serde_json::Value {
    /// See [The GeoJSON Format](https://geojson.org/).
    fn from(coord: Coordinate) -> Self {
        serde_json::json!({
            GEOJSON_TYPE_FIELD: GEOJSON_POINT_TYPE,
            GEOJSON_COORDINATES_FIELD: [
                coord.lat.as_float().0,
                coord.long.as_float().0
            ]
        })
    }
}

#[cfg(feature = "geojson")]
impl TryFrom<serde_json::Value> for Coordinate {
    type Error = crate::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value[GEOJSON_TYPE_FIELD] != GEOJSON_POINT_TYPE {
            return Err(crate::Error::InvalidCoordinate);
        }
        let coords = value[GEOJSON_COORDINATES_FIELD]
            .as_array()
            .ok_or(crate::Error::InvalidCoordinate)?;
        if coords.len() != 2 {
            return Err(crate::Error::InvalidCoordinate);
        }
        let lat_val: f64 = coords[0]
            .as_f64()
            .ok_or(crate::Error::InvalidNumericFormat(coords[0].to_string()))?;
        let lon_val: f64 = coords[1]
            .as_f64()
            .ok_or(crate::Error::InvalidNumericFormat(coords[1].to_string()))?;
        let lat = Latitude::try_from(lat_val)?;
        let lon = Longitude::try_from(lon_val)?;
        Ok(Coordinate::new(lat, lon))
    }
}
