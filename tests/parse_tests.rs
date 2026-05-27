//! Integration tests for `parse::parse_str`.
//!
//! Tests are organised by the format being exercised, then by the success /
//! error axis within that format.

use lat_long::{
    Coordinate, Error, Latitude, Longitude,
    parse::{self, Parsed, Value},
};

// ---------------------------------------------------------------------------
// Helper predicates
// ---------------------------------------------------------------------------

fn is_unknown(r: &Parsed) -> bool {
    matches!(r, Parsed::Angle(Value::Unknown(_)))
}

fn is_latitude_value(r: &Parsed) -> bool {
    matches!(r, Parsed::Angle(Value::Latitude(_)))
}

fn is_longitude_value(r: &Parsed) -> bool {
    matches!(r, Parsed::Angle(Value::Longitude(_)))
}

fn is_coordinate(r: &Parsed) -> bool {
    matches!(r, Parsed::Coordinate(_))
}

// ---------------------------------------------------------------------------
// Decimal format — single values
// ---------------------------------------------------------------------------

#[test]
fn decimal_positive() {
    let r = parse::parse_str("48.858222").unwrap();
    assert!(is_unknown(&r));
}

#[test]
fn decimal_with_plus_sign() {
    let r = parse::parse_str("+48.858222").unwrap();
    assert!(is_unknown(&r));
}

#[test]
fn decimal_negative() {
    let r = parse::parse_str("-48.858222").unwrap();
    assert!(is_unknown(&r));
}

#[test]
fn decimal_short_fraction() {
    // 1 fractional digit is fine.
    assert!(parse::parse_str("48.9").is_ok());
}

#[test]
fn decimal_leading_zero_allowed() {
    // Single leading zero is allowed (e.g. "048.9").
    assert!(parse::parse_str("048.9").is_ok());
}

#[test]
fn decimal_err_no_decimal_point() {
    // "48" has no dot → InvalidNumericFormat.
    assert_eq!(
        parse::parse_str("48"),
        Err(Error::InvalidNumericFormat("48".to_string()))
    );
}

#[test]
fn decimal_err_too_many_integer_digits() {
    // Four integer digits → InvalidNumericFormat.
    assert!(matches!(
        parse::parse_str("0048.9"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn decimal_err_leading_whitespace() {
    assert_eq!(
        parse::parse_str(" 48.858222"),
        Err(Error::InvalidWhitespace(" 48.858222".to_string()))
    );
}

#[test]
fn decimal_err_trailing_whitespace() {
    assert_eq!(
        parse::parse_str("48.858222 "),
        Err(Error::InvalidWhitespace("48.858222 ".to_string()))
    );
}

#[test]
fn decimal_err_space_after_sign() {
    // "- 48.858222" — whitespace between sign and digits.
    assert!(matches!(
        parse::parse_str("- 48.858222"),
        Err(Error::InvalidWhitespace(_))
    ));
}

// ---------------------------------------------------------------------------
// Signed DMS format — single values
// ---------------------------------------------------------------------------

#[test]
fn signed_dms_positive() {
    let r = parse::parse_str("48° 51′ 29.600000″").unwrap();
    assert!(is_unknown(&r), "got {r:?}");
}

#[test]
fn signed_dms_negative() {
    let r = parse::parse_str("-48° 51′ 29.600000″").unwrap();
    assert!(is_unknown(&r), "got {r:?}");
}

#[test]
fn signed_dms_no_whitespace_between_components() {
    // Whitespace between components is optional; none is also valid.
    let r = parse::parse_str("1°2′3.0″").unwrap();
    assert!(is_unknown(&r));
}

#[test]
fn signed_dms_err_invalid_minutes() {
    assert!(matches!(
        parse::parse_str("48° 61′ 29.600000″"),
        Err(Error::InvalidMinutes(61))
    ));
}

#[test]
fn signed_dms_err_invalid_seconds() {
    assert!(matches!(
        parse::parse_str("48° 51′ 61.0″"),
        Err(Error::InvalidSeconds(_))
    ));
}

// ---------------------------------------------------------------------------
// Labeled DMS format — single values
// ---------------------------------------------------------------------------

#[test]
fn labeled_dms_north() {
    let r = parse::parse_str("48° 51′ 29.600000″ N").unwrap();
    assert!(is_latitude_value(&r), "expected Latitude, got {r:?}");
}

#[test]
fn labeled_dms_south() {
    let r = parse::parse_str("48° 51′ 29.600000″ S").unwrap();
    assert!(is_latitude_value(&r), "expected Latitude, got {r:?}");
}

#[test]
fn labeled_dms_east() {
    let r = parse::parse_str("73° 59′ 8.400000″ E").unwrap();
    assert!(is_longitude_value(&r), "expected Longitude, got {r:?}");
}

#[test]
fn labeled_dms_west() {
    let r = parse::parse_str("73° 59′ 8.400000″ W").unwrap();
    assert!(is_longitude_value(&r), "expected Longitude, got {r:?}");
}

#[test]
fn labeled_dms_err_wrong_minutes_symbol() {
    // ASCII apostrophe instead of U+2032 PRIME.
    assert!(matches!(
        parse::parse_str("48° 51' 29.600000″ N"),
        Err(Error::InvalidCharacter(_, _)) | Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn labeled_dms_err_lowercase_direction() {
    // 'w' is not a valid direction (must be uppercase).
    assert!(matches!(
        parse::parse_str("48° 51′ 29.600000″ w"),
        Err(Error::InvalidCharacter(_, _))
    ));
}

#[test]
fn labeled_dms_err_unknown_direction() {
    assert!(matches!(
        parse::parse_str("48° 51′ 29.600000″ X"),
        Err(Error::InvalidCharacter(_, _))
    ));
}

// ---------------------------------------------------------------------------
// Bare DMS format — single values
// ---------------------------------------------------------------------------

#[test]
fn bare_dms_positive() {
    let r = parse::parse_str("+048:51:29.600000").unwrap();
    assert!(is_unknown(&r), "got {r:?}");
}

#[test]
fn bare_dms_negative() {
    let r = parse::parse_str("-073:59:08.400000").unwrap();
    assert!(is_unknown(&r), "got {r:?}");
}

#[test]
fn bare_dms_err_no_sign() {
    // Bare format requires a leading +/-.
    assert!(parse::parse_str("048:51:29.600000").is_err());
}

#[test]
fn bare_dms_err_wrong_degree_width() {
    // Degrees must be exactly 3 digits.
    assert!(matches!(
        parse::parse_str("+48:51:29.600000"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn bare_dms_err_wrong_minute_width() {
    // Minutes must be exactly 2 digits.
    assert!(matches!(
        parse::parse_str("+048:5:29.600000"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

// ---------------------------------------------------------------------------
// Coordinate pairs — decimal + decimal
// ---------------------------------------------------------------------------

#[test]
fn coord_decimal_decimal() {
    let r = parse::parse_str("48.858222, -73.985667").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_decimal_decimal_no_whitespace() {
    let r = parse::parse_str("48.858222,-73.985667").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_decimal_err_lat_out_of_range() {
    // 91.0° > 90° latitude limit.
    assert!(matches!(
        parse::parse_str("91.0, 0.0"),
        Err(Error::InvalidLatitudeDegrees(_)) | Err(Error::InvalidAngle(_, _))
    ));
}

#[test]
fn coord_decimal_err_lon_out_of_range() {
    // 181.0° > 180° longitude limit.
    assert!(matches!(
        parse::parse_str("0.0, 181.0"),
        Err(Error::InvalidLongitudeDegrees(_)) | Err(Error::InvalidAngle(_, _))
    ));
}

#[test]
fn coord_boundary_values_ok() {
    // Exactly ±90 lat and ±180 lon are valid.
    assert!(parse::parse_str("90.0, 0.0").is_ok());
    assert!(parse::parse_str("-90.0, 0.0").is_ok());
    assert!(parse::parse_str("0.0, 180.0").is_ok());
    assert!(parse::parse_str("0.0, -180.0").is_ok());
}

// ---------------------------------------------------------------------------
// Coordinate pairs — labeled DMS + labeled DMS
// ---------------------------------------------------------------------------

#[test]
fn coord_labeled_n_e() {
    let r = parse::parse_str("48° 51′ 29.600000″ N, 73° 59′ 8.400000″ E").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_labeled_n_w() {
    let r = parse::parse_str("48° 51′ 29.600000″ N, 73° 59′ 8.400000″ W").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_labeled_s_e() {
    let r = parse::parse_str("33° 52′ 0.000000″ S, 18° 25′ 0.000000″ E").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_labeled_err_e_in_lat_slot() {
    // E/W in the latitude slot.
    assert!(matches!(
        parse::parse_str("48° 51′ 29.600000″ E, 73° 59′ 8.400000″ W"),
        Err(Error::InvalidCharacter(_, _))
    ));
}

#[test]
fn coord_labeled_err_w_in_lat_slot() {
    assert!(matches!(
        parse::parse_str("48° 51′ 29.600000″ W, 73° 59′ 8.400000″ W"),
        Err(Error::InvalidCharacter(_, _))
    ));
}

#[test]
fn coord_labeled_err_n_in_lon_slot() {
    // N/S in the longitude slot.
    assert!(matches!(
        parse::parse_str("48° 51′ 29.600000″ N, 73° 59′ 8.400000″ N"),
        Err(Error::InvalidCharacter(_, _))
    ));
}

#[test]
fn coord_labeled_err_s_in_lon_slot() {
    assert!(matches!(
        parse::parse_str("48° 51′ 29.600000″ N, 73° 59′ 8.400000″ S"),
        Err(Error::InvalidCharacter(_, _))
    ));
}

// ---------------------------------------------------------------------------
// Coordinate pairs — signed DMS
// ---------------------------------------------------------------------------

#[test]
fn coord_signed_dms_both() {
    let r = parse::parse_str("48° 51′ 29.600000″, 73° 59′ 8.400000″").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_signed_dms_negative_lon() {
    let r = parse::parse_str("48° 51′ 29.600000″, -73° 59′ 8.400000″").unwrap();
    assert!(is_coordinate(&r));
}

// ---------------------------------------------------------------------------
// Coordinate pairs — bare DMS
// ---------------------------------------------------------------------------

#[test]
fn coord_bare_both_no_whitespace() {
    let r = parse::parse_str("+048:51:29.600000,-073:59:08.400000").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_bare_err_whitespace_around_comma() {
    // bare+bare must not have whitespace around the comma.
    assert!(matches!(
        parse::parse_str("+048:51:29.600000, -073:59:08.400000"),
        Err(Error::InvalidWhitespace(_))
    ));
}

// ---------------------------------------------------------------------------
// Coordinate pairs — mixed formats
// ---------------------------------------------------------------------------

#[test]
fn coord_mixed_bare_lat_and_labeled_lon() {
    let r = parse::parse_str("+048:51:29.600000, 73° 59′ 8.400000″ W").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_mixed_signed_dms_lat_and_decimal_lon() {
    let r = parse::parse_str("48° 51′ 29.600000″, 73.985667").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_mixed_decimal_lat_and_signed_dms_lon() {
    let r = parse::parse_str("48.858222, 73° 59′ 8.400000″").unwrap();
    assert!(is_coordinate(&r));
}

#[test]
fn coord_mixed_negative_decimal_lat_and_signed_dms_lon() {
    let r = parse::parse_str("-48.858222, 73° 59′ 8.400000″").unwrap();
    assert!(is_coordinate(&r));
}

// ---------------------------------------------------------------------------
// Error: too many commas
// ---------------------------------------------------------------------------

#[test]
fn err_three_values() {
    // Two commas → not a valid two-value coordinate.
    assert!(matches!(
        parse::parse_str("48.0, 0.0, 0.0"),
        Err(Error::InvalidCharacter(',', _))
    ));
}

// ---------------------------------------------------------------------------
// Error: empty string
// ---------------------------------------------------------------------------

#[test]
fn err_empty_string() {
    assert!(parse::parse_str("").is_err());
}

// ---------------------------------------------------------------------------
// Error: garbage
// ---------------------------------------------------------------------------

#[test]
fn err_garbage() {
    assert!(parse::parse_str("hello world").is_err());
}

// ---------------------------------------------------------------------------
// FromStr round-trips
// ---------------------------------------------------------------------------

#[test]
fn from_str_latitude_decimal() {
    let lat: Latitude = "45.5".parse().unwrap();
    assert!(lat.is_northern());
}

#[test]
fn from_str_latitude_labeled_n() {
    let lat: Latitude = "48° 51′ 29.600000″ N".parse().unwrap();
    assert!(lat.is_northern());
}

#[test]
fn from_str_latitude_labeled_s() {
    let lat: Latitude = "33° 52′ 0.000000″ S".parse().unwrap();
    assert!(lat.is_southern());
}

#[test]
fn from_str_latitude_err_longitude_direction() {
    // E direction → Value::Longitude → FromStr for Latitude rejects it.
    let result: Result<Latitude, _> = "48° 51′ 29.600000″ E".parse();
    assert!(result.is_err());
}

#[test]
fn from_str_longitude_decimal() {
    let lon: Longitude = "-73.9".parse().unwrap();
    assert!(lon.is_western());
}

#[test]
fn from_str_longitude_labeled_e() {
    let lon: Longitude = "73° 59′ 8.400000″ E".parse().unwrap();
    assert!(lon.is_eastern());
}

#[test]
fn from_str_longitude_labeled_w() {
    let lon: Longitude = "73° 59′ 8.400000″ W".parse().unwrap();
    assert!(lon.is_western());
}

#[test]
fn from_str_longitude_err_latitude_direction() {
    // N direction → Value::Latitude → FromStr for Longitude rejects it.
    let result: Result<Longitude, _> = "48° 51′ 29.600000″ N".parse();
    assert!(result.is_err());
}

#[test]
fn from_str_coordinate_labeled() {
    let coord: Coordinate = "48° 51′ 29.600000″ N, 73° 59′ 8.400000″ W".parse().unwrap();
    assert!(coord.latitude().is_northern());
    assert!(coord.longitude().is_western());
}

#[test]
fn from_str_coordinate_decimal() {
    let coord: Coordinate = "48.858222, -73.985667".parse().unwrap();
    assert!(coord.latitude().is_northern());
    assert!(coord.longitude().is_western());
}

#[test]
fn from_str_coordinate_bare() {
    let coord: Coordinate = "+048:51:29.600000,-073:59:08.400000".parse().unwrap();
    assert!(coord.latitude().is_northern());
    assert!(coord.longitude().is_western());
}

#[test]
fn from_str_coordinate_err_not_a_pair() {
    // A single-angle string → Parsed::Angle → FromStr for Coordinate rejects it.
    let result: Result<Coordinate, _> = "48.858222".parse();
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Helpers for value-level assertions (not just discriminant matching)
// ---------------------------------------------------------------------------

fn unwrap_unknown(r: Parsed) -> f64 {
    match r {
        Parsed::Angle(Value::Unknown(v)) => v.into_inner(),
        other => panic!("expected Parsed::Angle(Unknown), got {other:?}"),
    }
}

fn unwrap_coordinate(r: Parsed) -> Coordinate {
    match r {
        Parsed::Coordinate(c) => c,
        other => panic!("expected Parsed::Coordinate, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Decimal — additional malformed-input coverage
// ---------------------------------------------------------------------------

#[test]
fn decimal_negative_value_preserved() {
    // Regression: the negative sign must survive the parse — `-73.9` is *not* `+73.9`.
    let v = unwrap_unknown(parse::parse_str("-73.9").unwrap());
    assert!(v < 0.0, "expected negative value, got {v}");
    assert!((v - -73.9).abs() < 1e-9);
}

#[test]
fn decimal_positive_value_preserved() {
    let v = unwrap_unknown(parse::parse_str("48.5").unwrap());
    assert!((v - 48.5).abs() < 1e-9);
}

#[test]
fn decimal_explicit_plus_value_preserved() {
    let v = unwrap_unknown(parse::parse_str("+48.5").unwrap());
    assert!((v - 48.5).abs() < 1e-9);
}

#[test]
fn decimal_err_multiple_dots() {
    assert!(matches!(
        parse::parse_str("48.5.0"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn decimal_err_empty_integer_part() {
    // ".5" has no integer digits.
    assert!(matches!(
        parse::parse_str(".5"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn decimal_err_empty_fraction_part() {
    // "48." has a dot but no fractional digits.
    assert!(matches!(
        parse::parse_str("48."),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn decimal_err_lone_sign() {
    assert!(parse::parse_str("+").is_err());
    assert!(parse::parse_str("-").is_err());
}

#[test]
fn decimal_err_lone_dot() {
    assert!(matches!(
        parse::parse_str("."),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn decimal_err_non_digit_in_integer() {
    assert!(matches!(
        parse::parse_str("4a.5"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn decimal_err_non_digit_in_fraction() {
    assert!(matches!(
        parse::parse_str("48.5a"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

// ---------------------------------------------------------------------------
// Labeled decimal — happy paths
//
// Per the documented behavior table, labeled decimals return `Value::Unknown`
// (the direction letter is applied to the *sign*, not to the kind). Only
// labeled DMS produces `Value::Latitude` / `Value::Longitude`.
// ---------------------------------------------------------------------------

#[test]
fn labeled_decimal_north_no_fraction() {
    let v = unwrap_unknown(parse::parse_str("48N").unwrap());
    assert!((v - 48.0).abs() < 1e-9);
}

#[test]
fn labeled_decimal_south_with_fraction() {
    // S flips the sign.
    let v = unwrap_unknown(parse::parse_str("33.86S").unwrap());
    assert!((v - -33.86).abs() < 1e-9);
}

#[test]
fn labeled_decimal_east_no_fraction() {
    let v = unwrap_unknown(parse::parse_str("73E").unwrap());
    assert!((v - 73.0).abs() < 1e-9);
}

#[test]
fn labeled_decimal_west_with_fraction() {
    // W flips the sign.
    let v = unwrap_unknown(parse::parse_str("73.98W").unwrap());
    assert!((v - -73.98).abs() < 1e-9);
}

// ---------------------------------------------------------------------------
// Labeled decimal — error paths
// ---------------------------------------------------------------------------

#[test]
fn labeled_decimal_err_sign_and_direction_n() {
    // Sign and direction together are contradictory.
    assert!(matches!(
        parse::parse_str("-48.5N"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn labeled_decimal_err_sign_and_direction_e() {
    assert!(matches!(
        parse::parse_str("+73.9E"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn labeled_decimal_err_sign_and_direction_w() {
    assert!(matches!(
        parse::parse_str("-73.9W"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

// ---------------------------------------------------------------------------
// Signed DMS — additional coverage
// ---------------------------------------------------------------------------

#[test]
fn signed_dms_explicit_plus() {
    let r = parse::parse_str("+48° 51′ 29.6″").unwrap();
    assert!(is_unknown(&r));
}

#[test]
fn signed_dms_err_whitespace_after_sign() {
    assert!(matches!(
        parse::parse_str("- 48° 51′ 29.6″"),
        Err(Error::InvalidWhitespace(_))
    ));
}

#[test]
fn signed_dms_err_whitespace_before_degree_symbol() {
    assert!(matches!(
        parse::parse_str("48 ° 51′ 29.6″"),
        Err(Error::InvalidWhitespace(_))
    ));
}

#[test]
fn signed_dms_err_whitespace_before_minute_symbol() {
    assert!(matches!(
        parse::parse_str("48° 51 ′ 29.6″"),
        Err(Error::InvalidWhitespace(_))
    ));
}

#[test]
fn signed_dms_err_whitespace_before_second_symbol() {
    assert!(matches!(
        parse::parse_str("48° 51′ 29.6 ″"),
        Err(Error::InvalidWhitespace(_))
    ));
}

#[test]
fn signed_dms_err_seconds_without_fraction() {
    // `parse_seconds` requires a `.` in the seconds component.
    assert!(parse::parse_str("48° 51′ 29″").is_err());
}

#[test]
fn signed_dms_err_too_many_degree_digits() {
    assert!(matches!(
        parse::parse_str("1234° 0′ 0.0″"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

// ---------------------------------------------------------------------------
// Labeled DMS — additional coverage (boundaries and range errors)
// ---------------------------------------------------------------------------

#[test]
fn labeled_dms_boundary_90_north() {
    let r = parse::parse_str("90° 0′ 0.0″ N").unwrap();
    assert!(is_latitude_value(&r));
}

#[test]
fn labeled_dms_boundary_180_east() {
    let r = parse::parse_str("180° 0′ 0.0″ E").unwrap();
    assert!(is_longitude_value(&r));
}

#[test]
fn labeled_dms_err_lat_degrees_out_of_range() {
    // 91° N exceeds the 90° latitude limit.
    assert!(matches!(
        parse::parse_str("91° 0′ 0.0″ N"),
        Err(Error::InvalidLatitudeDegrees(_)) | Err(Error::InvalidAngle(_, _))
    ));
}

#[test]
fn labeled_dms_err_lon_degrees_out_of_range() {
    // 181° E exceeds the 180° longitude limit.
    assert!(matches!(
        parse::parse_str("181° 0′ 0.0″ E"),
        Err(Error::InvalidLongitudeDegrees(_)) | Err(Error::InvalidAngle(_, _))
    ));
}

#[test]
fn labeled_dms_err_invalid_minutes() {
    assert!(matches!(
        parse::parse_str("48° 60′ 0.0″ N"),
        Err(Error::InvalidMinutes(60))
    ));
}

#[test]
fn labeled_dms_err_invalid_seconds() {
    assert!(matches!(
        parse::parse_str("48° 0′ 60.0″ N"),
        Err(Error::InvalidSeconds(_))
    ));
}

#[test]
fn labeled_dms_err_whitespace_before_direction_is_optional() {
    // No whitespace between ″ and direction is also fine.
    let r = parse::parse_str("48° 51′ 29.6″N").unwrap();
    assert!(is_latitude_value(&r));
}

#[test]
fn labeled_dms_err_extra_chars_after_direction() {
    assert!(parse::parse_str("48° 51′ 29.6″ NX").is_err());
}

// ---------------------------------------------------------------------------
// Bare DMS — additional coverage
// ---------------------------------------------------------------------------

#[test]
fn bare_dms_err_too_few_seconds_fraction_digits() {
    // Bare seconds require ≥4 fractional digits.
    assert!(matches!(
        parse::parse_str("+048:51:29.600"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn bare_dms_err_seconds_missing_dot() {
    // Bare format demands a fractional seconds component.
    assert!(parse::parse_str("+048:51:29").is_err());
}

#[test]
fn bare_dms_err_seconds_wrong_int_width() {
    // Seconds integer part must be exactly 2 digits before the dot.
    assert!(matches!(
        parse::parse_str("+048:51:2.60000"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn bare_dms_err_non_digit_in_degrees() {
    assert!(matches!(
        parse::parse_str("+0a8:51:29.6000"),
        Err(Error::InvalidNumericFormat(_))
    ));
}

#[test]
fn bare_dms_negative_value_preserved() {
    let v = unwrap_unknown(parse::parse_str("-073:00:00.0000").unwrap());
    assert!((v - -73.0).abs() < 1e-9);
}

// ---------------------------------------------------------------------------
// Coordinate pairs — content assertions (not just discriminant matching)
// ---------------------------------------------------------------------------

#[test]
fn coord_decimal_values_correct() {
    let coord = unwrap_coordinate(parse::parse_str("48.858222, -73.985667").unwrap());
    assert!(coord.latitude().is_northern());
    assert!(coord.longitude().is_western());
    assert!((f64::from(coord.latitude()) - 48.858222).abs() < 1e-6);
    assert!((f64::from(coord.longitude()) - -73.985667).abs() < 1e-6);
}

#[test]
fn coord_labeled_values_correct() {
    let coord =
        unwrap_coordinate(parse::parse_str("48° 51′ 29.600000″ S, 73° 59′ 8.400000″ W").unwrap());
    assert!(coord.latitude().is_southern());
    assert!(coord.longitude().is_western());
}

// ---------------------------------------------------------------------------
// Coordinate pairs — malformed structural cases
// ---------------------------------------------------------------------------

#[test]
fn coord_err_empty_before_comma() {
    assert!(parse::parse_str(", 48.0").is_err());
}

#[test]
fn coord_err_empty_after_comma() {
    assert!(parse::parse_str("48.0,").is_err());
}

#[test]
fn coord_err_only_comma() {
    assert!(parse::parse_str(",").is_err());
}

#[test]
fn coord_err_only_whitespace_around_comma() {
    assert!(parse::parse_str(" , ").is_err());
}

// ---------------------------------------------------------------------------
// FromStr — out-of-range Unknown values rejected by the typed slot
// ---------------------------------------------------------------------------

#[test]
fn from_str_latitude_err_out_of_range_decimal() {
    // 95.0 parses as Unknown but fails latitude validation.
    let result: Result<Latitude, _> = "95.0".parse();
    assert!(result.is_err());
}

#[test]
fn from_str_longitude_err_out_of_range_decimal() {
    let result: Result<Longitude, _> = "200.0".parse();
    assert!(result.is_err());
}

#[test]
fn from_str_latitude_err_garbage() {
    let result: Result<Latitude, _> = "not a latitude".parse();
    assert!(result.is_err());
}

#[test]
fn from_str_longitude_err_garbage() {
    let result: Result<Longitude, _> = "not a longitude".parse();
    assert!(result.is_err());
}

#[test]
fn from_str_coordinate_err_garbage() {
    let result: Result<Coordinate, _> = "hello, world".parse();
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Round-trip via Display
// ---------------------------------------------------------------------------

#[test]
fn round_trip_latitude_decimal_display() {
    let lat: Latitude = "48.858222".parse().unwrap();
    let s = format!("{lat}");
    let lat2: Latitude = s.parse().unwrap();
    assert_eq!(lat, lat2);
}

#[test]
fn round_trip_longitude_decimal_display() {
    let lon: Longitude = "-73.985667".parse().unwrap();
    let s = format!("{lon}");
    let lon2: Longitude = s.parse().unwrap();
    assert_eq!(lon, lon2);
}

#[test]
fn round_trip_coordinate_decimal_display() {
    let coord: Coordinate = "48.858222, -73.985667".parse().unwrap();
    let s = format!("{coord}");
    let coord2: Coordinate = s.parse().unwrap();
    assert_eq!(coord, coord2);
}
