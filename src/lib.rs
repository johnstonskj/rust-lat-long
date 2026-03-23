//! Geographic latitude and longitude coordinate types.
//!
//! This crate provides strongly-typed [`Latitude`], [`Longitude`], and [`Coordinate`] values that are validated on
//! construction and carry their own display logic (decimal degrees **or** degrees–minutes–seconds). The goal is not
//! to provide a single, large, and potentially unwieldy "geo" crate, but rather a collection of small, focused crates
//! that can be used together or independently.
//!
//! ## Quick start
//!
//! ```rust
//! use lat_long::{Angle, Coordinate, Latitude, Longitude};
//!
//! let lat = Latitude::new(48, 51, 29.6).expect("valid latitude");
//! let lon = Longitude::new(2, 21, 7.6).expect("valid longitude");
//! let paris = Coordinate::new(lat, lon);
//!
//! // Decimal-degree display (default)
//! println!("{paris}");   // => 48.858222, 2.218778
//! // Degrees–minutes–seconds display (alternate flag)
//! println!("{paris:#}"); // => 48° 51' 29.6" N, 2° 21' 7.6" E
//! ```
//!
//! ```rust
//! use lat_long::{parse::{self, Parsed}, Coordinate};
//!
//! if let Ok(Parsed::Coordinate(london)) = parse::parse_str("51.522, -0.127") {
//!     println!("{london}"); // => 51.522, -0.127
//! }
//! ```
//!
//! ```rust,ignore
//! // Convert to URL, requires `url` feature flag
//! let url = url::Url::from(paris);
//! println!("{url}"); // => geo:48.858222,2.218778
//! ```
//!
//! ```rust,ignore
//! // Convert to JSON, requires `geojson` feature flag
//! let json = serde_json::Value::from(paris);
//! println!("{json}"); // => { "type": "Point", "coordinates": [48.858222,2.218778] }
//! ```
//!
//! ## Formatting
//!
//! The [`fmt`] module provides functionality for formatting and parsing coordinates.
//!
//! | `FormatKind`    | Format String | Positive             | Negative             |
//! |-----------------|---------------|----------------------|----------------------|
//! | `Decimal`       | `{}`          | 48.858222            | -48.858222           |
//! | `DmsSigned`     | `{:#}`        | 48° 51′ 29.600000″   | -48° 51′ 29.600000″  |
//! | `DmsLabeled`    | N/A           | 48° 51′ 29.600000″ N | 48° 51′ 29.600000″ S |
//! | `DmsBare`       | N/A           | +048:51:29.600000    | -048:51:29.600000    |
//!
//! Note that the `DmsBare` format is intended as a regular, easy-to-parse format for use in
//! data files, rather than as a human-readable format. In it`s coordinate pair form, it is
//! also the only format that does not allow whitespace around the comma separator.
//!
//! ## Parsing
//!
//! The [`parse`] module provides functionality for parsing coordinates. The parser accepts all of the
//! formats described above. The parser is also used by the implementation of `FromStr` for `Latitude`,
//! `Longitude`, and `Coordinate`.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//! ## References
//!
//! * [Latitude and longitude](https://en.wikipedia.org/wiki/Geographic_coordinate_system#Latitude_and_longitude)
//! * [WGS 84](https://en.wikipedia.org/wiki/World_Geodetic_System)

use std::fmt::{Debug, Display};
use std::hash::Hash;

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

pub trait Angle:
    Clone
    + Copy
    + Debug
    + Default
    + Display
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + TryFrom<OrderedFloat<f64>, Error = Error>
    + Into<OrderedFloat<f64>>
{
    const MIN: Self;
    const MAX: Self;

    /// Construct a new angle from degrees, minutes, and seconds.
    fn new(degrees: i32, minutes: u32, seconds: f32) -> Result<Self, Error>
    where
        Self: Sized;

    fn as_float(&self) -> OrderedFloat<f64> {
        (*self).into()
    }

    /// Returns `true` if the angle is exactly zero.
    fn is_zero(&self) -> bool {
        self.as_float() == inner::ZERO
    }

    /// Returns `true` if the angle is positive and non-zero.
    fn is_nonzero_positive(&self) -> bool {
        !self.is_zero() && self.as_float() > inner::ZERO
    }

    /// Returns `true` if the angle is negative and non-zero.
    fn is_nonzero_negative(&self) -> bool {
        !self.is_zero() && self.as_float() < inner::ZERO
    }

    /// The signed integer degrees component (carries the sign for negative angles).
    fn degrees(&self) -> i32 {
        inner::to_degrees_minutes_seconds(self.as_float()).0
    }

    /// The unsigned minutes component (always in `0..60`).
    fn minutes(&self) -> u32 {
        inner::to_degrees_minutes_seconds(self.as_float()).1
    }

    /// The unsigned seconds component (always in `0.0..60.0`).
    fn seconds(&self) -> f32 {
        inner::to_degrees_minutes_seconds(self.as_float()).2
    }

    /// Checked absolute value. Computes self.abs(), returning None if self == MIN.
    fn checked_abs(self) -> Option<Self>
    where
        Self: Sized,
    {
        if self == Self::MIN {
            None
        } else {
            Some(Self::try_from(OrderedFloat(self.into().0.abs())).unwrap())
        }
    }

    /// Computes the absolute value of self.
    ///
    /// Returns a tuple of the absolute version of self along with a boolean indicating whether an overflow happened.
    /// If self is the minimum value Self::MIN, then the minimum value will be returned again and true will be returned
    /// for an overflow happening.
    fn overflowing_abs(self) -> (Self, bool)
    where
        Self: Sized,
    {
        if self == Self::MIN {
            (self, true)
        } else {
            (
                Self::try_from(OrderedFloat(self.into().0.abs())).unwrap(),
                false,
            )
        }
    }

    /// Saturating absolute value. Computes self.abs(), returning MAX if self == MIN instead of overflowing.
    fn saturating_abs(self) -> Self
    where
        Self: Sized,
    {
        if self == Self::MIN {
            Self::MAX
        } else {
            Self::try_from(OrderedFloat(self.into().0.abs())).unwrap()
        }
    }

    /// Strict absolute value. Computes self.abs(), panicking if self == MIN.
    fn strict_abs(self) -> Self
    where
        Self: Sized,
    {
        if self == Self::MIN {
            panic!("attempt to take absolute value of the minimum value")
        } else {
            Self::try_from(OrderedFloat(self.into().0.abs())).unwrap()
        }
    }

    /// Unchecked absolute value. Computes self.abs(), assuming overflow cannot occur.
    ///
    /// Calling x.unchecked_abs() is semantically equivalent to calling x.checked_abs().unwrap_unchecked().
    ///
    /// If you’re just trying to avoid the panic in debug mode, then do not use this. Instead, you’re looking for wrapping_abs.
    fn unchecked_abs(self) -> Self
    where
        Self: Sized,
    {
        Self::try_from(OrderedFloat(self.into().0.abs())).unwrap()
    }

    /// Wrapping (modular) absolute value. Computes self.abs(), wrapping around at the boundary of the type.
    ///
    /// The only case where such wrapping can occur is when one takes the absolute value of the negative minimal
    /// value for the type; this is a positive value that is too large to represent in the type. In such a case,
    /// this function returns MIN itself.
    fn wrapping_abs(self) -> Self
    where
        Self: Sized,
    {
        if self == Self::MIN {
            Self::MIN
        } else {
            Self::try_from(OrderedFloat(self.into().0.abs())).unwrap()
        }
    }
}

// ---------------------------------------------------------------------------
// Internal Modules
// ---------------------------------------------------------------------------

mod inner;
pub mod parse;

// ---------------------------------------------------------------------------
// Public Modules & Exports
// ---------------------------------------------------------------------------

pub mod coord;
pub use coord::Coordinate;
pub mod error;
pub use error::Error;
pub mod fmt;
pub mod lat;
pub use lat::Latitude;
pub mod long;
pub use long::Longitude;
use ordered_float::OrderedFloat;
