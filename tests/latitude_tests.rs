use lat_long::{
    Angle, Error, Latitude,
    fmt::{FormatOptions, Formatter},
    lat::{
        ANTARCTIC_CIRCLE, ARCTIC_CIRCLE, EQUATOR, NORTH_POLE, SOUTH_POLE, TROPIC_OF_CANCER,
        TROPIC_OF_CAPRICORN,
    },
};

// --- Latitude construction ---

#[test]
fn test_lat_valid_values() {
    assert!(Latitude::new(0, 0, 0.0).is_ok());
    assert!(Latitude::new(45, 30, 0.0).is_ok());
    assert!(Latitude::new(-90, 0, 0.0).is_ok());
    assert!(Latitude::new(90, 0, 0.0).is_ok());
    assert!(Latitude::new(-45, 59, 59.9).is_ok());
}

#[test]
fn test_lat_invalid_degrees() {
    assert_eq!(
        Latitude::new(91, 0, 0.0),
        Err(Error::InvalidLatitudeDegrees(91))
    );
    assert_eq!(
        Latitude::new(-91, 0, 0.0),
        Err(Error::InvalidLatitudeDegrees(-91))
    );
}

#[test]
fn test_lat_invalid_minutes() {
    assert_eq!(Latitude::new(45, 61, 0.0), Err(Error::InvalidMinutes(61)));
}

#[test]
fn test_lat_invalid_seconds() {
    assert_eq!(Latitude::new(45, 0, 61.0), Err(Error::InvalidSeconds(61.0)));
    assert_eq!(Latitude::new(45, 0, -1.0), Err(Error::InvalidSeconds(-1.0)));
}

/// Regression test: verify fix to integer division bug in `degrees_to_float`.
#[test]
fn test_lat_degrees_to_float_minutes_precision() {
    // 0° 30′ 0″ should be 0.5 degrees, not 0.0 (integer division bug)
    let lat = Latitude::new(0, 30, 0.0).unwrap();
    let s = format!("{lat}");
    assert!(
        s.contains("0.5"),
        "expected 0.5 degrees for 0°30′, got: {s}"
    );
}

// --- Display ---

#[test]
fn test_lat_display() {
    let lat = Latitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(format!("{lat}"), "-45");
}

#[test]
fn test_lat_display_alt() {
    let lat = Latitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(format!("{lat:#}"), "-45° 0′ 0.000000″");
}

// --- Formatting ---

#[test]
fn test_lat_format_decimal_positive() {
    let lat = Latitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::decimal().with_precision(2)),
        "45.00"
    );
}

#[test]
fn test_lat_format_decimal_negative() {
    let lat = Latitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::decimal().with_precision(2)),
        "-45.00"
    );
}

#[test]
fn test_lat_format_dms_signed_positive() {
    let lat = Latitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::dms_signed().with_precision(2)),
        "45° 0′ 0.00″"
    );
}

#[test]
fn test_lat_format_dms_signed_negative() {
    let lat = Latitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::dms_signed().with_precision(2)),
        "-45° 0′ 0.00″"
    );
}

#[test]
fn test_lat_format_dms_labeled_positive() {
    let lat = Latitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::dms_labeled().with_precision(2)),
        "45° 0′ 0.00″ N"
    );
}

#[test]
fn test_lat_format_dms_labeled_negative() {
    let lat = Latitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::dms_labeled().with_precision(2)),
        "45° 0′ 0.00″ S"
    );
}

#[test]
fn test_lat_format_dms_bare_positive() {
    let lat = Latitude::new(45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::dms_bare().with_precision(2)),
        "+045:00:00.0000"
    );
}

#[test]
fn test_lat_format_dms_bare_negative() {
    let lat = Latitude::new(-45, 0, 0.0).unwrap();
    assert_eq!(
        lat.to_formatted_string(&FormatOptions::dms_bare().with_precision(2)),
        "-045:00:00.0000"
    );
}

// --- Hemisphere predicates ---

#[test]
fn test_lat_is_northern() {
    let north = Latitude::new(45, 0, 0.0).unwrap();
    assert!(north.is_northern());
    assert!(!north.is_southern());
    assert!(!north.is_on_equator());
}

#[test]
fn test_lat_is_southern() {
    let south = Latitude::new(-45, 0, 0.0).unwrap();
    assert!(south.is_southern());
    assert!(!south.is_northern());
    assert!(!south.is_on_equator());
}

#[test]
fn test_lat_is_on_equator() {
    let equator = Latitude::new(0, 0, 0.0).unwrap();
    assert!(equator.is_on_equator());
    assert!(!equator.is_northern());
    assert!(!equator.is_southern());
}

// --- Latitude constants ---

#[test]
fn test_lat_constants_values() {
    assert_eq!(NORTH_POLE.degrees(), 90);
    assert_eq!(SOUTH_POLE.degrees(), -90);
    assert_eq!(ARCTIC_CIRCLE.degrees(), 66);
    assert_eq!(ANTARCTIC_CIRCLE.degrees(), -66);
    assert_eq!(TROPIC_OF_CANCER.degrees(), 23);
    assert_eq!(TROPIC_OF_CAPRICORN.degrees(), -23);
    assert_eq!(EQUATOR.degrees(), 0);
}

#[test]
fn test_lat_constants_ordering() {
    assert!(NORTH_POLE > ARCTIC_CIRCLE);
    assert!(ARCTIC_CIRCLE > TROPIC_OF_CANCER);
    assert!(TROPIC_OF_CANCER > EQUATOR);
    assert!(EQUATOR > TROPIC_OF_CAPRICORN);
    assert!(TROPIC_OF_CAPRICORN > ANTARCTIC_CIRCLE);
    assert!(ANTARCTIC_CIRCLE > SOUTH_POLE);
}

// --- DMS accessors ---

#[test]
fn test_lat_dms_accessors() {
    let lat = Latitude::new(48, 51, 0.0).unwrap();
    assert_eq!(lat.degrees(), 48);
    assert_eq!(lat.minutes(), 51);
    assert!(
        lat.seconds() < 1e-4,
        "expected ~0s for whole minutes, got {}",
        lat.seconds()
    );
}

#[test]
fn test_lat_dms_accessors_with_seconds() {
    let lat = Latitude::new(48, 51, 30.0).unwrap();
    assert_eq!(lat.degrees(), 48);
    assert_eq!(lat.minutes(), 51);
    assert!(
        (lat.seconds() - 30.0_f32).abs() < 1e-3,
        "expected ~30.0s, got {}",
        lat.seconds()
    );
}

#[test]
fn test_lat_dms_accessors_southern() {
    // Negative decimal degree: degrees() is negative, minutes() is positive.
    let lat = Latitude::new(-33, 52, 0.0).unwrap();
    assert_eq!(lat.degrees(), -33);
    assert_eq!(lat.minutes(), 52);
    assert!(
        lat.seconds() < 1e-4,
        "expected ~0s for whole minutes, got {}",
        lat.seconds()
    );
}

#[test]
fn test_lat_dms_accessors_zero_degrees() {
    // 0° 30' should report 0 degrees and 30 minutes.
    let lat = Latitude::new(0, 30, 0.0).unwrap();
    assert_eq!(lat.degrees(), 0);
    assert_eq!(lat.minutes(), 30);
}

// --- Zone predicates ---

#[test]
fn test_lat_zone_polar() {
    // North Pole and Arctic Circle are polar.
    assert!(NORTH_POLE.is_polar());
    assert!(NORTH_POLE.is_arctic());
    assert!(!NORTH_POLE.is_antarctic());

    assert!(ARCTIC_CIRCLE.is_arctic());
    assert!(ARCTIC_CIRCLE.is_polar());

    assert!(SOUTH_POLE.is_polar());
    assert!(SOUTH_POLE.is_antarctic());
    assert!(!SOUTH_POLE.is_arctic());

    assert!(ANTARCTIC_CIRCLE.is_antarctic());
    assert!(ANTARCTIC_CIRCLE.is_polar());

    // Mid-latitude (e.g. London) is not polar.
    let london = Latitude::new(51, 30, 0.0).unwrap();
    assert!(!london.is_polar());
    assert!(!london.is_arctic());
    assert!(!london.is_antarctic());
}

#[test]
fn test_lat_zone_tropical() {
    // Equator is tropical (between tropics).
    assert!(!EQUATOR.is_tropic_of_cancer());
    assert!(!EQUATOR.is_tropic_of_capricorn());
    assert!(!EQUATOR.is_tropical());

    // Tropic of Cancer boundary itself.
    assert!(TROPIC_OF_CANCER.is_tropic_of_cancer());
    assert!(!TROPIC_OF_CANCER.is_tropic_of_capricorn());
    assert!(TROPIC_OF_CANCER.is_tropical());

    // Tropic of Capricorn boundary itself.
    assert!(!TROPIC_OF_CAPRICORN.is_tropic_of_cancer());
    assert!(TROPIC_OF_CAPRICORN.is_tropic_of_capricorn());
    assert!(TROPIC_OF_CAPRICORN.is_tropical());

    // Far north is "at or above Tropic of Cancer" — is_tropic_of_cancer returns true.
    assert!(NORTH_POLE.is_tropic_of_cancer());
    assert!(NORTH_POLE.is_tropical());

    // Far south is "at or below Tropic of Capricorn".
    assert!(SOUTH_POLE.is_tropic_of_capricorn());
    assert!(SOUTH_POLE.is_tropical());
}

#[test]
fn test_lat_zone_non_polar_non_tropical() {
    // 45° N — northern mid-latitude, not polar, not tropical.
    let mid = Latitude::new(45, 0, 0.0).unwrap();
    assert!(!mid.is_polar());
    assert!(!mid.is_arctic());
    assert!(!mid.is_antarctic());
    // 45° is above the Tropic of Cancer (23.5°), so is_tropic_of_cancer is true.
    assert!(mid.is_tropic_of_cancer());
}

// --- TryFrom conversions ---

#[test]
fn test_lat_try_from_f64_valid() {
    let lat: Latitude = 45.5_f64.try_into().unwrap();
    assert!(lat.is_northern());

    let south: Latitude = (-33.5_f64).try_into().unwrap();
    assert!(south.is_southern());

    let equator: Latitude = 0.0_f64.try_into().unwrap();
    assert!(equator.is_on_equator());
}

#[test]
fn test_lat_try_from_f64_invalid() {
    let result: Result<Latitude, _> = 91.0_f64.try_into();
    assert!(result.is_err());

    let result: Result<Latitude, _> = (-91.0_f64).try_into();
    assert!(result.is_err());
}

#[test]
fn test_lat_try_from_float_angle() {
    use ordered_float::OrderedFloat;
    // Round-trip: Latitude → f64 → OrderedFloat → Latitude.
    let src = Latitude::new(45, 30, 0.0).unwrap();
    let float = OrderedFloat(f64::from(src));
    let lat: Latitude = float.try_into().unwrap();
    assert!(lat.is_northern());

    // A value in the valid longitude range but outside latitude range.
    let result: Result<Latitude, _> = OrderedFloat(95.0_f64).try_into();
    assert!(result.is_err());
}
