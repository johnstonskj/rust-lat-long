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

use ordered_float::OrderedFloat;
use std::fmt::{Debug, Display};
use std::hash::Hash;

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

///
/// Shared interface for angular geographic values ([`Latitude`] and [`Longitude`]).
///
/// `Angle` captures the operations that apply to both kinds of geographic
/// angle: construction from degrees–minutes–seconds, conversion to and from
/// decimal degrees and radians, and the various flavours of absolute value
/// (mirroring those provided by Rust's primitive integer types).
///
/// Implementations are guaranteed to be `Copy`, totally ordered (via
/// [`OrderedFloat`]), and hashable — making them suitable as keys in
/// `BTreeMap`/`HashMap` collections.
///
/// # Examples
///
/// ```rust
/// use lat_long::{Angle, Latitude};
///
/// // Construct from degrees / minutes / seconds.
/// let lat = Latitude::new(45, 30, 0.0).expect("valid latitude");
///
/// // Decompose back into components.
/// assert_eq!(lat.degrees(), 45);
/// assert_eq!(lat.minutes(), 30);
///
/// // Convert to and from radians.
/// let radians = lat.to_radians();
/// let round_trip = Latitude::from_radians(radians).unwrap();
/// assert_eq!(lat, round_trip);
/// ```
///
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
    + TryFrom<f64, Error = Error>
    + TryFrom<OrderedFloat<f64>, Error = Error>
    + Into<OrderedFloat<f64>>
    + Into<f64>
{
    ///
    /// The minimum legal value of this angle.
    ///
    /// For [`Latitude`] this is `-90°`; for [`Longitude`] this is `-180°`.
    ///
    const MIN: Self;

    ///
    /// The maximum legal value of this angle.
    ///
    /// For [`Latitude`] this is `+90°`; for [`Longitude`] this is `+180°`.
    ///
    const MAX: Self;

    ///
    /// Construct a new angle from degrees, minutes, and seconds.
    ///
    /// For negative angles, only `degrees` carries the sign — `minutes` and
    /// `seconds` are always non-negative.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] variant if any component is out of range:
    /// [`Error::InvalidLatitudeDegrees`] / [`Error::InvalidLongitudeDegrees`]
    /// for an out-of-range degrees value, [`Error::InvalidMinutes`] for
    /// `minutes ≥ 60`, or [`Error::InvalidSeconds`] for `seconds < 0.0` or
    /// `seconds ≥ 60.0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Latitude, Longitude};
    ///
    /// let lat = Latitude::new(45, 30, 0.0).unwrap();
    /// let lon = Longitude::new(-122, 19, 59.0).unwrap();
    /// assert!(lat.is_nonzero_positive());
    /// assert!(lon.is_nonzero_negative());
    ///
    /// // Out-of-range values are rejected.
    /// assert!(Latitude::new(91, 0, 0.0).is_err());
    /// assert!(Longitude::new(0, 60, 0.0).is_err());
    /// ```
    ///
    fn new(degrees: i32, minutes: u32, seconds: f32) -> Result<Self, Error>
    where
        Self: Sized;

    ///
    /// Returns the underlying decimal-degree value as an [`OrderedFloat<f64>`].
    ///
    /// Useful when interoperating with collections that require total ordering
    /// (e.g. `BTreeMap`) or with APIs that use [`OrderedFloat`] directly.
    ///
    fn as_float(&self) -> OrderedFloat<f64> {
        (*self).into()
    }

    ///
    /// Returns the angle converted from decimal degrees to radians.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Latitude};
    ///
    /// let lat = Latitude::new(180 / 2, 0, 0.0).unwrap(); // 90°
    /// // 90° is π/2 radians.
    /// assert!((lat.to_radians() - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    /// ```
    ///
    fn to_radians(&self) -> f64 {
        self.as_float().0.to_radians()
    }

    ///
    /// Construct an angle from a value in radians.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the resulting decimal-degree value is outside
    /// the legal range for the target type, or is not finite.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Latitude};
    ///
    /// let lat = Latitude::from_radians(std::f64::consts::FRAC_PI_4).unwrap();
    /// assert!((f64::from(lat) - 45.0).abs() < 1e-12);
    /// ```
    ///
    fn from_radians(radians: f64) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::try_from(OrderedFloat(radians.to_degrees()))
    }

    ///
    /// Returns `true` if the angle is exactly zero.
    ///
    fn is_zero(&self) -> bool {
        self.as_float() == inner::ZERO
    }

    ///
    /// Returns `true` if the angle is positive and non-zero.
    ///
    fn is_nonzero_positive(&self) -> bool {
        !self.is_zero() && self.as_float() > inner::ZERO
    }

    ///
    /// Returns `true` if the angle is negative and non-zero.
    ///
    fn is_nonzero_negative(&self) -> bool {
        !self.is_zero() && self.as_float() < inner::ZERO
    }

    ///
    /// The signed integer degrees component (carries the sign for negative angles).
    ///
    fn degrees(&self) -> i32 {
        inner::to_degrees_minutes_seconds(self.as_float()).0
    }

    ///
    /// The unsigned minutes component (always in `0..60`).
    ///
    fn minutes(&self) -> u32 {
        inner::to_degrees_minutes_seconds(self.as_float()).1
    }

    ///
    /// The unsigned seconds component (always in `0.0..60.0`).
    ///
    fn seconds(&self) -> f32 {
        inner::to_degrees_minutes_seconds(self.as_float()).2
    }

    ///
    /// Returns the absolute value of this angle.
    ///
    /// Because both `-MIN` and `+MAX` round-trip safely for [`Latitude`] and
    /// [`Longitude`] (they are symmetric about zero), this method never panics
    /// for valid input. For symmetry with the primitive-integer `*_abs`
    /// family, prefer [`checked_abs`](Self::checked_abs),
    /// [`saturating_abs`](Self::saturating_abs), etc., when you want to
    /// document overflow intent explicitly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Latitude};
    ///
    /// let south = Latitude::new(-30, 0, 0.0).unwrap();
    /// assert_eq!(south.abs(), Latitude::new(30, 0, 0.0).unwrap());
    /// ```
    ///
    fn abs(self) -> Self
    where
        Self: Sized,
    {
        Self::try_from(OrderedFloat(self.as_float().0.abs())).unwrap()
    }

    ///
    /// Returns this angle taken modulo [`MAX`](Self::MAX).
    ///
    /// Useful for wrapping a longitude into the canonical `(-180°, +180°]`
    /// range, or a latitude into `(-90°, +90°]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lat_long::{Angle, Longitude};
    ///
    /// // 180° is already at the antimeridian; modulo MAX yields 0°.
    /// let lon = Longitude::new(180, 0, 0.0).unwrap();
    /// assert!(lon.modulo_max().is_zero());
    /// ```
    ///
    fn modulo_max(self) -> Self
    where
        Self: Sized,
    {
        Self::try_from(self.as_float() % Self::MAX.as_float()).unwrap()
    }

    ///
    /// Checked absolute value. Computes `self.abs()`, returning `None` if `self == MIN`.
    ///
    fn checked_abs(self) -> Option<Self>
    where
        Self: Sized,
    {
        if self == Self::MIN {
            None
        } else {
            Some(Self::try_from(OrderedFloat(self.as_float().0.abs())).unwrap())
        }
    }

    ///
    /// Computes the absolute value of self.
    ///
    /// Returns a tuple of the absolute version of `self` along with a boolean
    /// indicating whether an overflow happened. If `self` is the minimum value
    /// `Self::MIN``, then the minimum value will be returned again and `true`
    /// will be returned for an overflow happening.
    ///
    fn overflowing_abs(self) -> (Self, bool)
    where
        Self: Sized,
    {
        if self == Self::MIN {
            (self, true)
        } else {
            (
                Self::try_from(OrderedFloat(self.as_float().0.abs())).unwrap(),
                false,
            )
        }
    }

    ///
    /// Saturating absolute value. Computes `self.abs()`, returning `MAX`
    /// if `self == MIN` instead of overflowing.
    ///
    fn saturating_abs(self) -> Self
    where
        Self: Sized,
    {
        if self == Self::MIN {
            Self::MAX
        } else {
            Self::try_from(OrderedFloat(self.as_float().abs())).unwrap()
        }
    }

    ///
    /// Strict absolute value. Computes `self.abs()`, panicking if `self == MIN`.
    ///
    fn strict_abs(self) -> Self
    where
        Self: Sized,
    {
        if self == Self::MIN {
            panic!("attempt to take absolute value of the minimum value")
        } else {
            Self::try_from(OrderedFloat(self.as_float().0.abs())).unwrap()
        }
    }

    ///
    /// Unchecked absolute value. Computes s`elf.abs()`, assuming overflow cannot occur.
    ///
    /// Calling `x.unchecked_abs() `is semantically equivalent to calling
    /// `x.checked_abs().unwrap_unchecked()`.
    ///
    /// If you’re just trying to avoid the panic in debug mode, then do not use
    /// this. Instead, you’re looking for `wrapping_abs`.
    ///
    fn unchecked_abs(self) -> Self
    where
        Self: Sized,
    {
        Self::try_from(OrderedFloat(self.as_float().0.abs())).unwrap()
    }

    ///
    /// Wrapping (modular) absolute value. Computes `self.abs()`, wrapping around at
    /// the boundary of the type.
    ///
    /// The only case where such wrapping can occur is when one takes the absolute
    /// value of the negative minimal value for the type; this is a positive value
    /// that is too large to represent in the type. In such a case, this function
    /// returns `MIN` itself.
    ///
    fn wrapping_abs(self) -> Self
    where
        Self: Sized,
    {
        if self == Self::MIN {
            Self::MIN
        } else {
            Self::try_from(OrderedFloat(self.as_float().0.abs())).unwrap()
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

#[cfg(feature = "elevation")]
pub mod elevation;
#[cfg(feature = "elevation")]
pub use elevation::{CoordinateWithElevation, Elevation};

pub mod coord;
pub use coord::Coordinate;
pub mod error;
pub use error::Error;
pub mod fmt;
pub mod latitude;
pub use latitude::Latitude;
pub mod longitude;
pub use longitude::Longitude;
