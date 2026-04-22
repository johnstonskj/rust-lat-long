//! This module provides the [`Parsed`] enum and [`parse_str`] function.
//!
//! The [`parse_str`] function should proceed according to the following grammar:
//!
//! ```bnf
//! <coordinate>    ::= <bare_pair> | <non_bare_pair>
//! <bare_pair>     ::= <bare_dms> "," <bare_dms>
//! <non_bare_pair> ::= ( <latitude> <separator> <longitude> )
//!                     | ( <bare_dms> <separator> <longitude> )
//!                     | ( <latitude> <separator> <bare_dms> )
//!
//! <latitude>      ::= <decimal> | <signed_dms> | <labeled_dms>
//! <longitude>     ::= <decimal> | <signed_dms> | <labeled_dms>
//! <separator>     ::= WHITESPACE* "," WHITESPACE*
//! <decimal>       ::= <sign>? <digits> "." <digits>
//! <signed_dms>    ::= <sign>? <degs> WHITESPACE* <mins> WHITESPACE* <secs>
//! <labeled_dms>   ::= <degs> WHITESPACE* <mins> WHITESPACE* <secs> WHITESPACE* <direction>
//! <bare_dms>      ::= <sign> <bare_degs> ":" <bare_mins> ":" <bare_secs>
//!
//! <degs>          ::= <digits> "°"
//! <mins>          ::= <digits> "′"
//! <secs>          ::= <digits> "." <digits> "″"
//! <bare_degs>     ::= <digit> <digit> <digit>
//! <bare_mins>     ::= <digit> <digit>
//! <bare_secs>     ::= <digit> <digit> "." <digit> <digit> <digit> <digit>+
//!
//! <direction>     ::= "N" | "S" | "E" | "W"
//! <sign>          ::= "+" | "-"
//! <digit>         ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
//! <digits>        ::= <digit>+
//! ```
//!
//! # Format Notes
//!
//! 1. Strings **must not** have leading or trailing whitespace. [`Error::InvalidWhitespace`]
//! 2. Whitespace **must not** appear between the *sign* and the *degrees* value. [`Error::InvalidWhitespace`]
//! 3. *Degree*, *minute*, and *seconds* values have a maximum number of integer digits, 3, 2, and 2 respectively. [`Error::InvalidNumericFormat`]
//! 4. The *degrees* symbol **must** be '°', (Unicode `U+00B0` `DEGREE SIGN`). [`Error::InvalidCharacter`]
//! 5. The *minutes* symbol **must** be '′' (Unicode `U+2032` `PRIME`). [`Error::InvalidCharacter`]
//! 6. The *seconds* symbol **must** be '″' (Unicode `U+2033` `DOUBLE PRIME`). [`Error::InvalidCharacter`]
//! 7. Whitespace **must not** appear between the *degrees*, *minutes*, and *seconds* values and their corresponding symbols. [`Error::InvalidWhitespace`]
//! 8. Labeled format **must** have a *direction* character 'N', 'S', 'E', or 'W' at the end of the string, this character is case sensitive. [`Error::InvalidCharacter`]
//! 9. Bare format **must** start with the *sign* character [+|-] at the beginning of the string. [`Error::InvalidNumericFormat`]
//! 10. The latitude and longitude values of a coordinate **may** be specified in different formats.
//! 11. The latitude and longitude values of a coordinate **must** separated by a single comma `,` (*separator*).
//! 12. If either latitude **and** longitude are specified in a non-bare format, the *separator* **may** have whitespace before and after: `\s*,\s*`.
//! 12. If both latitude **and** longitude are specified in bare format, the *separator* **must not** have leading or trailing whitespace. [`Error::InvalidNumericFormat`]
//!
//! # Parsed Examples
//!
//! | Input String                              | Match           | Result                        |
//! |-------------------------------------------|-----------------|-------------------------------|
//! | 48.858222                                 | Decimal         | Ok(Value(Unknown))            |
//! | +48.858222                                | Decimal         | Ok(Value(Unknown))            |
//! | 48.858                                    | Decimal         | Ok(Value(Unknown))            |
//! | 48.9                                      | Decimal         | Ok(Value(Unknown))            |
//! | 48                                        | Decimal         | Error(InvalidNumericFormat)   |
//! | 048.9                                     | Decimal         | Ok(Value(Unknown))            |
//! | 0048.9                                    | Decimal         | Error(InvalidNumericFormat)   |
//! | -48.858222                                | Decimal         | Ok(Value(Unknown))            |
//! | " 48.858222"                              | Decimal         | Error(InvalidWhitespace)      |
//! | "48.858222 "                              | Decimal         | Error(InvalidWhitespace)      |
//! | - 48.858222                               | Decimal         | Error(InvalidCharacter)       |
//! | 48° 51′ 29.600000″                        | Signed DMS      | Ok(Value(Unknown))            |
//! | -48° 51′ 29.600000″                       | Signed DMS      | Ok(Value(Unknown))            |
//! | 48° 51′ 29.600000″                        | Signed DMS      | Ok(Value(Unknown))            |
//! | 48° 51' 29.600000″ N                      | Labeled DMS     | Error(InvalidCharacter)       |
//! | 48° 51′ 29.600000″ S                      | Labeled DMS     | Ok(Value(Latitude))           |
//! | 48° 51′ 29.600000″ E                      | Labeled DMS     | Ok(Value(Longitude))          |
//! | 48° 51′ 29.600000″ W                      | Labeled DMS     | Ok(Value(Longitude))          |
//! | 48° 51′ 29.600000″ w                      | Labeled DMS     | Error(InvalidCharacter)       |
//! | +048:51:29.600000                         | Bare DMS        | Ok(Value(Unknown))            |
//! | -048:51:29.600000                         | Bare DMS        | Ok(Value(Unknown))            |
//! | 91, 0, 0.0                                | Signed DMS      | Error(InvalidDegrees)         |
//! | 90, 61, 0.0                               | Signed DMS      | Error(InvalidMinutes)         |
//! | 90, 0, 61.0                               | Signed DMS      | Error(InvalidSeconds)         |
//! | 180, 1, 0.0                               | Signed DMS      | Error(InvalidAngle)           |
//! | 48° 51′ 29.600000″, 73° 59′ 8.400000″     | Signed+Signed   | Ok(Coordinate)                |
//! | 48° 51′ 29.600000″ N, 73° 59′ 8.400000″ E | Labeled+Labeled | Ok(Coordinate)                |
//! | 48° 51′ 29.600000″ W, 73° 59′ 8.400000″ N | Labeled+Labeled | Error(InvalidLatitude)        |
//! | 48° 51′ 29.600000″ X, 73° 59′ 8.400000″ Y | Labeled+Labeled | Error(InvalidCharacter)       |
//! | 48.858222, -73.985667                     | Decimal+Decimal | Ok(Coordinate)                |
//! | +048:51:29.600000, 73° 59′ 8.400000″      | Bare+Signed     | Ok(Coordinate)                |
//! | 48° 51′ 29.600000″, 73.985667             | Signed+Decimal  | Ok(Coordinate)                |
//! | 48.858222, 73° 59′ 8.400000″              | Decimal+Signed  | Ok(Coordinate)                |
//! | 48° 51′ 29.600000″, -73.985667            | Signed+Decimal  | Ok(Coordinate)                |
//! | -48.858222, 73° 59′ 8.400000″             | Decimal+Signed  | Ok(Coordinate)                |
//! | +048:51:29.600000,-073:59:08.400000       | Bare+Bare       | Ok(Coordinate)                |
//! | +048:51:29.600000, -073:59:08.400000      | Bare+Bare       | Error(InvalidWhitespace)      |
//!
//! # Code Examples
//!
//! Parse individual angles:
//!
//! ```rust
//! use lat_long::parse;
//!
//! assert!(parse::parse_str("48.858222").is_ok());
//! assert!(parse::parse_str("-73.985667").is_ok());
//! assert!(parse::parse_str("48° 51′ 29.600000″ N").is_ok());
//! assert!(parse::parse_str("73° 59′  8.400000″ W").is_ok());
//! assert!(parse::parse_str("+048:51:29.600000").is_ok());
//! assert!(parse::parse_str("-073:59:08.400000").is_ok());
//! ```
//!
//! Parse coordinates:
//!
//! ```rust
//! use lat_long::parse;
//!
//! assert!(parse::parse_str("48.858222, -73.985667").is_ok());
//! assert!(parse::parse_str("48° 51′ 29.600000″ N, 73° 59′ 8.400000″ W").is_ok());
//! assert!(parse::parse_str("+048:51:29.600000,-073:59:08.400000").is_ok());
//! assert!(parse::parse_str("+048:51:29.600000, 73° 59′ 8.400000″ W").is_ok());
//! ```

use crate::{Coordinate, Error, Latitude, Longitude, inner};
use ordered_float::OrderedFloat;

// ---------------------------------------------------------------------------
// Public Types
// ---------------------------------------------------------------------------

/// The result of a successful [`parse_str`] call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Parsed {
    Angle(Value),
    Coordinate(Coordinate),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    /// A value whose direction (latitude vs longitude) is not specified
    /// by its format (decimal or signed/bare DMS).
    Unknown(OrderedFloat<f64>),
    /// A labeled DMS value with direction N or S — known to be a latitude.
    Latitude(Latitude),
    /// A labeled DMS value with direction E or W — known to be a longitude.
    Longitude(Longitude),
}

// ---------------------------------------------------------------------------
// Public Functions
// ---------------------------------------------------------------------------

/// Parse a string into a [`Parsed`] enum.
///
/// Accepts all four angle formats (decimal, signed DMS, labeled DMS, bare DMS)
/// as individual values or as a comma-separated coordinate pair. See the
/// module-level documentation for the full grammar and format rules.
pub fn parse_str(s: &str) -> Result<Parsed, Error> {
    // Rule 1: no leading or trailing whitespace.
    if s.starts_with(|c: char| c.is_ascii_whitespace())
        || s.ends_with(|c: char| c.is_ascii_whitespace())
    {
        return Err(Error::InvalidWhitespace(s.to_string()));
    }

    if s.is_empty() {
        return Err(Error::InvalidNumericFormat(s.to_string()));
    }

    // Look for a coordinate-pair comma.
    match find_comma(s) {
        Some(comma_pos) => parse_pair(s, comma_pos),
        None => parse_single(s).map(Parsed::Angle),
    }
}

// ---------------------------------------------------------------------------
// Private helpers: top-level dispatch
// ---------------------------------------------------------------------------

/// Find the byte index of the first ASCII comma in `s`.
fn find_comma(s: &str) -> Option<usize> {
    s.find(',')
}

/// Parse a single-angle string (no comma present). Returns the [`Value`].
fn parse_single(s: &str) -> Result<Value, Error> {
    // Try in order: labeled DMS (has °...″ + direction letter), signed DMS
    // (has °...″ without direction), bare DMS (starts with +/-NNN:MM:SS),
    // decimal.
    if let Some(result) = try_labeled_dms(s) {
        return result;
    }
    if let Some(result) = try_signed_dms(s) {
        return result.map(Value::Unknown);
    }
    if let Some(result) = try_bare_dms(s) {
        return result.map(Value::Unknown);
    }
    try_decimal(s).map(Value::Unknown)
}

/// Parse a comma-separated coordinate pair. `comma_pos` is the byte index of
/// the first comma in `s`.
fn parse_pair(s: &str, comma_pos: usize) -> Result<Parsed, Error> {
    let lat_src = &s[..comma_pos];
    let after_comma = &s[comma_pos + 1..];

    // Detect whitespace around the comma.
    let has_pre_ws = lat_src.ends_with(|c: char| c.is_ascii_whitespace());
    let has_post_ws = after_comma.starts_with(|c: char| c.is_ascii_whitespace());
    let comma_ws = has_pre_ws || has_post_ws;

    let lat_src = lat_src.trim_end();
    let lon_src = after_comma.trim_start();

    // Guard: no more commas allowed in the longitude part.
    if lon_src.contains(',') {
        return Err(Error::InvalidCharacter(',', s.to_string()));
    }

    // Determine which format(s) each side uses.
    let lat_is_bare = is_bare_dms(lat_src);
    let lon_is_bare = is_bare_dms(lon_src);

    // Rule: bare+bare must have no whitespace around the comma.
    if lat_is_bare && lon_is_bare && comma_ws {
        return Err(Error::InvalidWhitespace(s.to_string()));
    }

    // Parse the latitude slot.
    let lat: Latitude = parse_as_latitude(lat_src)?;
    // Parse the longitude slot.
    let lon: Longitude = parse_as_longitude(lon_src)?;

    Ok(Parsed::Coordinate(Coordinate::new(lat, lon)))
}

// ---------------------------------------------------------------------------
// Slot-typed parsers used for coordinate pairs
// ---------------------------------------------------------------------------

/// Parse `s` as the latitude slot of a coordinate pair.
///
/// Accepts: decimal, signed DMS, bare DMS (all produce `Unknown` → validated
/// as latitude), or labeled DMS with N/S.
/// Rejects: labeled DMS with E/W.
fn parse_as_latitude(s: &str) -> Result<Latitude, Error> {
    if let Some(result) = try_labeled_dms(s) {
        return match result? {
            Value::Latitude(lat) => Ok(lat),
            Value::Longitude(_) => {
                // E or W direction in the latitude slot.
                let dir_char = s.chars().last().unwrap_or('?');
                Err(Error::InvalidCharacter(dir_char, s.to_string()))
            }
            Value::Unknown(_) => unreachable!(),
        };
    }
    // Signed, bare, or decimal — parse as float then validate latitude range.
    let f = parse_as_float(s)?;
    Latitude::try_from(f).map_err(|_| {
        let deg = inner::to_degrees_minutes_seconds(f).0;
        Error::InvalidLatitudeDegrees(deg)
    })
}

/// Parse `s` as the longitude slot of a coordinate pair.
///
/// Accepts: decimal, signed DMS, bare DMS (all produce `Unknown` → validated
/// as longitude), or labeled DMS with E/W.
/// Rejects: labeled DMS with N/S.
fn parse_as_longitude(s: &str) -> Result<Longitude, Error> {
    if let Some(result) = try_labeled_dms(s) {
        return match result? {
            Value::Longitude(lon) => Ok(lon),
            Value::Latitude(_) => {
                // N or S direction in the longitude slot.
                let dir_char = s.chars().last().unwrap_or('?');
                Err(Error::InvalidCharacter(dir_char, s.to_string()))
            }
            Value::Unknown(_) => unreachable!(),
        };
    }
    let f = parse_as_float(s)?;
    Longitude::try_from(f).map_err(|_| {
        let deg = inner::to_degrees_minutes_seconds(f).0;
        Error::InvalidLongitudeDegrees(deg)
    })
}

/// Parse any non-labeled format (decimal, signed DMS, bare DMS) into a raw
/// float. Used when parsing a coordinate slot without a direction letter.
fn parse_as_float(s: &str) -> Result<OrderedFloat<f64>, Error> {
    if let Some(result) = try_signed_dms(s) {
        return result;
    }
    if let Some(result) = try_bare_dms(s) {
        return result;
    }
    try_decimal(s)
}

// ---------------------------------------------------------------------------
// Format detectors / sub-parsers
// ---------------------------------------------------------------------------

/// Returns `true` if `s` looks like a bare-DMS token (`+NNN:MM:SS.sss…`).
fn is_bare_dms(s: &str) -> bool {
    matches!(s.as_bytes().first(), Some(b'+') | Some(b'-')) && s.contains(':')
}

/// `^(?<sign>[-+])?(?<degrees>\d{1,3})°\s*(?<minutes>\d{1,2})′\s*(?<seconds>\d{1,2}\.\d+)″$`
///
/// Returns `None` if the string doesn't contain the `°` symbol (not this
/// format at all). Returns `Some(Err(_))` if it looks like this format but
/// is malformed.
///
/// Returns `Some(Ok(Value::Latitude))` for N/S direction,
/// `Some(Ok(Value::Longitude))` for E/W, `None` if no direction letter is
/// found (caller should try signed DMS next).
fn try_labeled_dms(s: &str) -> Option<Result<Value, Error>> {
    // Quick reject: must contain the degrees symbol.
    if !s.contains('°') {
        return None;
    }

    // The labeled variant does NOT start with a sign character — the polarity
    // is encoded in the direction letter.  If we see a leading +/- it could
    // be signed DMS; let the signed parser handle it.
    if matches!(s.as_bytes().first(), Some(b'+') | Some(b'-')) {
        return None;
    }

    let (deg_str, rest) = consume_up_to(s, '°')?;

    // Rule 7: no whitespace between the degree value and the ° symbol.
    if deg_str.ends_with(|c: char| c.is_ascii_whitespace()) {
        return Some(Err(Error::InvalidWhitespace(s.to_string())));
    }

    let rest = skip_whitespace(rest);
    let (min_str, rest) = consume_up_to(rest, '′')?;

    // Rule 7: no whitespace before ′.
    if min_str.ends_with(|c: char| c.is_ascii_whitespace()) {
        return Some(Err(Error::InvalidWhitespace(s.to_string())));
    }

    let rest = skip_whitespace(rest);
    let (sec_str, rest) = consume_up_to(rest, '″')?;

    // Rule 7: no whitespace before ″.
    if sec_str.ends_with(|c: char| c.is_ascii_whitespace()) {
        return Some(Err(Error::InvalidWhitespace(s.to_string())));
    }

    // What remains after ″ must be optional whitespace + exactly one direction
    // character (N/S/E/W) OR nothing (→ this is signed DMS, not labeled).
    let rest = rest.trim();
    if rest.is_empty() {
        // No direction letter — this is actually a signed DMS value; let the
        // signed parser handle it.
        return None;
    }

    // Validate direction character.
    let direction = match rest {
        "N" | "S" | "E" | "W" => rest,
        other => {
            let bad = other.chars().next().unwrap_or('?');
            return Some(Err(Error::InvalidCharacter(bad, s.to_string())));
        }
    };

    // Parse the numeric components (positive; sign comes from direction).
    let degrees = match parse_degrees(deg_str, 1, 3, false) {
        Some(d) => d,
        None => {
            return Some(Err(Error::InvalidNumericFormat(deg_str.to_string())));
        }
    };
    let minutes = match parse_minutes(min_str) {
        Some(m) => m,
        None => {
            return Some(Err(Error::InvalidNumericFormat(min_str.to_string())));
        }
    };
    let seconds = match parse_seconds(sec_str) {
        Some(t) => t,
        None => {
            return Some(Err(Error::InvalidNumericFormat(sec_str.to_string())));
        }
    };

    let neg = matches!(direction, "S" | "W");
    let signed_degrees = if neg { -degrees } else { degrees };

    let float = match inner::from_degrees_minutes_seconds(signed_degrees, minutes, seconds) {
        Ok(f) => f,
        Err(e) => return Some(Err(e)),
    };

    match direction {
        "N" | "S" => match Latitude::try_from(float) {
            Ok(lat) => Some(Ok(Value::Latitude(lat))),
            Err(_) => Some(Err(Error::InvalidLatitudeDegrees(
                inner::to_degrees_minutes_seconds(float).0,
            ))),
        },
        "E" | "W" => match Longitude::try_from(float) {
            Ok(lon) => Some(Ok(Value::Longitude(lon))),
            Err(_) => Some(Err(Error::InvalidLongitudeDegrees(
                inner::to_degrees_minutes_seconds(float).0,
            ))),
        },
        _ => unreachable!(),
    }
}

/// `^(?<sign>[-+])?(?<degrees>\d{1,3})°\s*(?<minutes>\d{1,2})′\s*(?<seconds>\d{1,2}\.\d+)″$`
///
/// Returns `None` if the string doesn't look like a signed DMS value.
fn try_signed_dms(s: &str) -> Option<Result<OrderedFloat<f64>, Error>> {
    if !s.contains('°') {
        return None;
    }

    // Optional leading sign.
    let (neg, s_inner) = consume_sign(s);

    // Rule 2: if we consumed a sign, the very next character must NOT be whitespace.
    if neg && s_inner.starts_with(|c: char| c.is_ascii_whitespace()) {
        return Some(Err(Error::InvalidWhitespace(s.to_string())));
    }

    let (deg_str, rest) = consume_up_to(s_inner, '°')?;

    // Rule 7: no whitespace before °.
    if deg_str.ends_with(|c: char| c.is_ascii_whitespace()) {
        return Some(Err(Error::InvalidWhitespace(s.to_string())));
    }

    let rest = skip_whitespace(rest);
    let (min_str, rest) = consume_up_to(rest, '′')?;

    if min_str.ends_with(|c: char| c.is_ascii_whitespace()) {
        return Some(Err(Error::InvalidWhitespace(s.to_string())));
    }

    let rest = skip_whitespace(rest);
    let (sec_str, rest) = consume_up_to(rest, '″')?;

    if sec_str.ends_with(|c: char| c.is_ascii_whitespace()) {
        return Some(Err(Error::InvalidWhitespace(s.to_string())));
    }

    // After ″ there must be nothing left for the signed variant.
    if !rest.trim().is_empty() {
        // There's a direction letter — this is labeled DMS, not signed.
        return None;
    }

    let degrees = match parse_degrees(deg_str, 1, 3, neg) {
        Some(d) => d,
        None => return Some(Err(Error::InvalidNumericFormat(deg_str.to_string()))),
    };
    let minutes = match parse_minutes(min_str) {
        Some(m) => m,
        None => return Some(Err(Error::InvalidNumericFormat(min_str.to_string()))),
    };
    let seconds = match parse_seconds(sec_str) {
        Some(t) => t,
        None => return Some(Err(Error::InvalidNumericFormat(sec_str.to_string()))),
    };

    Some(inner::from_degrees_minutes_seconds(
        degrees, minutes, seconds,
    ))
}

/// `^(?<sign>[-+])(?<degrees>\d{3}):(?<minutes>\d{2}):(?<seconds>\d{2}\.\d{4,})$`
///
/// Returns `None` if the string doesn't start with a sign followed by digits
/// and colons.
fn try_bare_dms(s: &str) -> Option<Result<OrderedFloat<f64>, Error>> {
    // Must start with mandatory sign.
    let neg = match s.as_bytes().first()? {
        b'+' => false,
        b'-' => true,
        _ => return None,
    };
    let s_inner = &s[1..];

    // Must contain at least two colons.
    if !s_inner.contains(':') {
        return None;
    }

    let (deg_str, rest) = consume_up_to(s_inner, ':')?;
    let (min_str, sec_str) = consume_up_to(rest, ':')?;

    // Validate lengths: exactly 3 degree digits, exactly 2 minute digits.
    if deg_str.len() != 3 || min_str.len() != 2 {
        return Some(Err(Error::InvalidNumericFormat(s.to_string())));
    }

    // Seconds must be `DD.DDDD+` — at least 4 fractional digits.
    let dot_pos = sec_str.find('.')?;
    if dot_pos != 2 || sec_str.len() < dot_pos + 1 + 4 {
        return Some(Err(Error::InvalidNumericFormat(s.to_string())));
    }

    let degrees = match parse_degrees(deg_str, 3, 3, neg) {
        Some(d) => d,
        None => return Some(Err(Error::InvalidNumericFormat(s.to_string()))),
    };
    let minutes = match parse_minutes(min_str) {
        Some(m) => m,
        None => return Some(Err(Error::InvalidNumericFormat(s.to_string()))),
    };
    let seconds = match parse_seconds(sec_str) {
        Some(t) => t,
        None => return Some(Err(Error::InvalidNumericFormat(s.to_string()))),
    };

    Some(inner::from_degrees_minutes_seconds(
        degrees, minutes, seconds,
    ))
}

/// `^(?<sign>[-+])?(?<int>\d{1,3})\.(?<frac>\d+)$`
///
/// Returns an error (not `None`) on obvious format violations so callers can
/// produce a good diagnostic.
fn try_decimal(s: &str) -> Result<OrderedFloat<f64>, Error> {
    // Rule 2: check for sign followed by whitespace.
    let (neg, rest) = consume_sign(s);
    if neg && rest.starts_with(|c: char| c.is_ascii_whitespace()) {
        return Err(Error::InvalidWhitespace(s.to_string()));
    }

    // Must contain exactly one dot.
    let dot = rest
        .find('.')
        .ok_or_else(|| Error::InvalidNumericFormat(s.to_string()))?;

    let int_part = &rest[..dot];
    let frac_part = &rest[dot + 1..];

    // Integer part: 1–3 digits.
    if int_part.is_empty() || int_part.len() > 3 || !int_part.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::InvalidNumericFormat(s.to_string()));
    }
    // Fractional part: ≥1 digit.
    if frac_part.is_empty() || !frac_part.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::InvalidNumericFormat(s.to_string()));
    }

    let int_val = parse_u32_digits(int_part.as_bytes())
        .ok_or_else(|| Error::InvalidNumericFormat(s.to_string()))?;
    let frac_val = parse_fraction(frac_part.as_bytes())
        .ok_or_else(|| Error::InvalidNumericFormat(s.to_string()))?;

    let magnitude = int_val as f64 + frac_val;
    let signed = if neg { -magnitude } else { magnitude };
    if signed.is_infinite() || signed.is_nan() {
        Err(Error::InvalidNumericValue(signed))
    } else {
        Ok(OrderedFloat(signed))
    }
}

// ---------------------------------------------------------------------------
// Sub-parsers (pure, no allocation)
// ---------------------------------------------------------------------------

/// Consume an optional leading `+` or `-`. Returns `(is_negative, rest_of_str)`.
fn consume_sign(s: &str) -> (bool, &str) {
    match s.as_bytes().first() {
        Some(b'+') => (false, &s[1..]),
        Some(b'-') => (true, &s[1..]),
        _ => (false, s),
    }
}

/// Return the slice before and after the first occurrence of Unicode `delim`.
/// Returns `None` if the delimiter is not found.
fn consume_up_to(s: &str, delim: char) -> Option<(&str, &str)> {
    let pos = s.find(delim)?;
    Some((&s[..pos], &s[pos + delim.len_utf8()..]))
}

/// Skip leading ASCII whitespace.
fn skip_whitespace(s: &str) -> &str {
    s.trim_start_matches(|c: char| c.is_ascii_whitespace())
}

/// Parse a degree string with `min_len..=max_len` digit count.
/// `neg` folds the sign into the return value.
fn parse_degrees(s: &str, min_len: usize, max_len: usize, neg: bool) -> Option<i32> {
    if s.len() < min_len || s.len() > max_len || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let v = parse_u32_digits(s.as_bytes())? as i32;
    Some(if neg { -v } else { v })
}

/// Parse a minutes string (1–2 digits).
fn parse_minutes(s: &str) -> Option<u32> {
    if s.is_empty() || s.len() > 2 || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    parse_u32_digits(s.as_bytes())
}

/// Parse a seconds string of the form `\d{1,2}\.\d+`.
fn parse_seconds(s: &str) -> Option<f32> {
    let dot = s.find('.')?;
    let int_part = &s[..dot];
    let frac_part = &s[dot + 1..];
    if int_part.is_empty()
        || int_part.len() > 2
        || frac_part.is_empty()
        || !int_part.bytes().all(|b| b.is_ascii_digit())
        || !frac_part.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let int_val = parse_u32_digits(int_part.as_bytes())?;
    let frac_val = parse_fraction(frac_part.as_bytes())?;
    Some((int_val as f64 + frac_val) as f32)
}

/// Accumulate ASCII decimal digits into a `u32`. Returns `None` on non-digit
/// bytes or overflow.
fn parse_u32_digits(bytes: &[u8]) -> Option<u32> {
    let mut acc: u32 = 0;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        acc = acc.checked_mul(10)?.checked_add((b - b'0') as u32)?;
    }
    Some(acc)
}

/// Convert the fractional-digit bytes after a `.` into a `f64` in `[0, 1)`.
fn parse_fraction(bytes: &[u8]) -> Option<f64> {
    let mut acc: f64 = 0.0;
    let mut place: f64 = 0.1;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        acc += (b - b'0') as f64 * place;
        place *= 0.1;
    }
    Some(acc)
}
