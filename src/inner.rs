//! This module provides the [`Angle`] type, and associated constants.

use crate::Error;
use ordered_float::OrderedFloat;

// ---------------------------------------------------------------------------
// Public Constants
// ---------------------------------------------------------------------------

pub const ZERO: OrderedFloat<f64> = OrderedFloat(0.0);

// ---------------------------------------------------------------------------
// Public Functions
// ---------------------------------------------------------------------------

const MINUTES_PER_DEGREE: f64 = 60.0;
const SECONDS_PER_MINUTE: f64 = 60.0;
const SECONDS_PER_DEGREE: f64 = MINUTES_PER_DEGREE * SECONDS_PER_MINUTE;

/// Construct a new `Angle` from degrees, minutes, and seconds.
///
///
pub(crate) const fn from_degrees_minutes_seconds(
    degrees: i32,
    minutes: u32,
    seconds: f32,
) -> Result<OrderedFloat<f64>, Error> {
    if minutes >= MINUTES_PER_DEGREE as u32 {
        return Err(Error::InvalidMinutes(minutes));
    }
    if seconds.is_sign_negative() || seconds >= SECONDS_PER_MINUTE as f32 {
        return Err(Error::InvalidSeconds(seconds));
    }
    Ok(OrderedFloat(to_decimal_degrees(degrees, minutes, seconds)))
}

/// Convert a (degrees, minutes, seconds) tuple into a decimal-degree float.
///
/// For negative locations (south latitude / west longitude), `degrees` is
/// negative and minutes/seconds are always non-negative. The formula applied
/// is `sign(degrees) × (|degrees| + minutes/60 + seconds/3600)`.
pub(crate) const fn to_decimal_degrees(degrees: i32, minutes: u32, seconds: f32) -> f64 {
    let abs_degs = degrees.abs() as f64;
    let fmins = minutes as f64 / MINUTES_PER_DEGREE;
    let fsecs = seconds as f64 / SECONDS_PER_DEGREE;
    let float = abs_degs + fmins + fsecs;
    // Restore the sign of the original degrees component.
    if degrees.is_negative() { -float } else { float }
}

/// Decompose a decimal-degree float into (degrees, minutes, seconds).
///
/// The sign is carried only in `degrees`; `minutes` and `seconds` are always
/// non-negative.
/// Decompose this angle into `(degrees, minutes, seconds)`. The sign is
/// carried only in `degrees`; `minutes` and `seconds` are always non-negative.
pub(crate) const fn to_degrees_minutes_seconds(angle: OrderedFloat<f64>) -> (i32, u32, f32) {
    let float = angle.0;
    let negative = float < 0.0;
    let abs = if negative { -float } else { float };
    let degrees_abs = abs.trunc() as i32;
    let remainder = abs.fract() * MINUTES_PER_DEGREE;
    let minutes = remainder.trunc() as u32;
    let seconds = (remainder.fract() * SECONDS_PER_MINUTE) as f32;
    let degrees = if negative { -degrees_abs } else { degrees_abs };
    (degrees, minutes, seconds)
}
