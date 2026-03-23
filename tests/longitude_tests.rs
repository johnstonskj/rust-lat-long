use lat_long::{
    Angle, Error, Longitude,
    fmt::{FormatOptions, Formatter},
    long::{ANTI_MERIDIAN, INTERNATIONAL_REFERENCE_MERIDIAN},
};

// --- Longitude construction ---

#[test]
fn test_long_valid_values() {
    assert!(Longitude::new(0, 0, 0.0).is_ok());
    assert!(Longitude::new(120, 30, 0.0).is_ok());
    assert!(Longitude::new(-180, 0, 0.0).is_ok());
    assert!(Longitude::new(180, 0, 0.0).is_ok());
}

#[test]
fn test_long_invalid_degrees() {
    assert_eq!(
        Longitude::new(181, 0, 0.0),
        Err(Error::InvalidLongitudeDegrees(181))
    );
    assert_eq!(
        Longitude::new(-181, 0, 0.0),
        Err(Error::InvalidLongitudeDegrees(-181))
    );
}

#[test]
fn test_long_invalid_minutes() {
    assert_eq!(Longitude::new(45, 61, 0.0), Err(Error::InvalidMinutes(61)));
}

#[test]
fn test_long_invalid_seconds() {
    assert_eq!(
        Longitude::new(45, 0, 61.0),
        Err(Error::InvalidSeconds(61.0))
    );
    assert_eq!(
        Longitude::new(45, 0, -1.0),
        Err(Error::InvalidSeconds(-1.0))
    );
}

/// Regression test: verify the bug where `Longitude::new` used to
/// emit `InvalidLatitudeDegrees`.
#[test]
fn test_long_error_variant_is_longitude_not_latitude() {
    let err = Longitude::new(200, 0, 0.0).unwrap_err();
    assert!(
        matches!(err, Error::InvalidLongitudeDegrees(_)),
        "expected InvalidLongitudeDegrees, got {err:?}"
    );
}

// --- Display ---

#[test]
fn test_long_display() {
    let lon = Longitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(format!("{lon}"), "-45");
}

#[test]
fn test_long_display_alt() {
    let lon = Longitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(format!("{lon:#}"), "-45° 0′ 0.000000″");
}

// --- Formatting ---

#[test]
fn test_long_format_decimal_positive() {
    let lon = Longitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::decimal().with_precision(2)),
        "45.00"
    );
}

#[test]
fn test_long_format_decimal_negative() {
    let lon = Longitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::decimal().with_precision(2)),
        "-45.00"
    );
}

#[test]
fn test_long_format_dms_signed_positive() {
    let lon = Longitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::dms_signed().with_precision(2)),
        "45° 0′ 0.00″"
    );
}

#[test]
fn test_long_format_dms_signed_negative() {
    let lon = Longitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::dms_signed().with_precision(2)),
        "-45° 0′ 0.00″"
    );
}

#[test]
fn test_long_format_dms_labeled_positive() {
    let lon = Longitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::dms_labeled().with_precision(2)),
        "45° 0′ 0.00″ E"
    );
}

#[test]
fn test_long_format_dms_labeled_negative() {
    let lon = Longitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::dms_labeled().with_precision(2)),
        "45° 0′ 0.00″ W"
    );
}

#[test]
fn test_long_format_dms_bare_positive() {
    let lon = Longitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::dms_bare().with_precision(2)),
        "+045:00:00.0000"
    );
}

#[test]
fn test_long_format_dms_bare_negative() {
    let lon = Longitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lon.to_formatted_string(&FormatOptions::dms_bare().with_precision(2)),
        "-045:00:00.0000"
    );
}

// --- Hemisphere predicates ---

#[test]
fn test_long_is_eastern() {
    let east = Longitude::new(10, 0, 0.0).unwrap();
    assert!(east.is_eastern());
    assert!(!east.is_western());
}

#[test]
fn test_long_is_western() {
    let west = Longitude::new(-10, 0, 0.0).unwrap();
    assert!(west.is_western());
    assert!(!west.is_eastern());
}

#[test]
fn test_long_is_on_irm() {
    let meridian = Longitude::new(0, 0, 0.0).unwrap();
    assert!(meridian.is_on_international_reference_meridian());
    assert!(!meridian.is_eastern());
    assert!(!meridian.is_western());
}

// --- Longitude constants ---

#[test]
fn test_long_constants_values() {
    assert_eq!(INTERNATIONAL_REFERENCE_MERIDIAN.degrees(), 0);
    assert!(INTERNATIONAL_REFERENCE_MERIDIAN.is_on_international_reference_meridian());
    assert_eq!(ANTI_MERIDIAN.degrees(), 180);
    assert!(ANTI_MERIDIAN.is_eastern());
}

// --- DMS accessors: Longitude ---

#[test]
fn test_long_dms_accessors() {
    let lon = Longitude::new(2, 21, 0.0).unwrap();
    assert_eq!(lon.degrees(), 2);
    assert_eq!(lon.minutes(), 21);
    assert!(
        lon.seconds() < 1e-4,
        "expected ~0s for whole minutes, got {}",
        lon.seconds()
    );
}

#[test]
fn test_long_dms_accessors_with_seconds() {
    let lon = Longitude::new(2, 21, 30.0).unwrap();
    assert_eq!(lon.degrees(), 2);
    assert_eq!(lon.minutes(), 21);
    assert!(
        (lon.seconds() - 30.0_f32).abs() < 1e-3,
        "expected ~30.0s, got {}",
        lon.seconds()
    );
}

#[test]
fn test_long_dms_accessors_western() {
    let lon = Longitude::new(-73, 56, 0.0).unwrap();
    assert_eq!(lon.degrees(), -73);
    assert_eq!(lon.minutes(), 56);
    assert!(
        lon.seconds() < 1e-4,
        "expected ~0s for whole minutes, got {}",
        lon.seconds()
    );
}

#[test]
fn test_long_dms_accessors_with_seconds_western() {
    // New York City longitude: 74° 0′ 23″ W
    let lon = Longitude::new(-74, 0, 23.0).unwrap();
    assert_eq!(lon.degrees(), -74);
    assert_eq!(lon.minutes(), 0);
    assert!(
        (lon.seconds() - 23.0_f32).abs() < 1e-3,
        "expected ~23.0s, got {}",
        lon.seconds()
    );
}

// --- TryFrom conversions ---

#[test]
fn test_long_try_from_f64_valid() {
    let lon: Longitude = 120.5_f64.try_into().unwrap();
    assert!(lon.is_eastern());

    let west: Longitude = (-73.9_f64).try_into().unwrap();
    assert!(west.is_western());

    let irm: Longitude = 0.0_f64.try_into().unwrap();
    assert!(irm.is_on_international_reference_meridian());
}

#[test]
fn test_long_try_from_f64_invalid() {
    let result: Result<Longitude, _> = 181.0_f64.try_into();
    assert!(result.is_err());

    let result: Result<Longitude, _> = (-181.0_f64).try_into();
    assert!(result.is_err());
}

#[test]
fn test_long_try_from_float_angle() {
    use ordered_float::OrderedFloat;
    // Round-trip: Longitude → f64 → OrderedFloat → Longitude.
    let src = Longitude::new(120, 30, 0.0).unwrap();
    let float = OrderedFloat(f64::from(src));
    let lon: Longitude = float.try_into().unwrap();
    assert!(lon.is_eastern());

    // 185° is outside the valid longitude range.
    let result: Result<Longitude, _> = 185.0_f64.try_into();
    assert!(result.is_err());
}
