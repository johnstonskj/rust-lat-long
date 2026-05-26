#![cfg(feature = "elevation")]

use std::str::FromStr;
use lat_long::{Angle, Coordinate, CoordinateWithElevation, Elevation, Error, Latitude, Longitude, elv};
use uom::si::{f64::Length, length};

// --- Elevation construction and helpers ---

#[test]
fn test_elevation_zero_and_value() {
    let elevation = Elevation::zero();

    assert!(elevation.is_zero());
    assert_eq!(elevation.value(), 0.0);
    assert_eq!(format!("{elevation}"), "0 m");
}

#[test]
fn test_elevation_constructors_and_unit_conversion() {
    let meters = Elevation::meters(10.0);
    let centimeters = Elevation::centimeters(1000.0);
    let kilometers = Elevation::kilometers(0.01);

    assert_eq!(meters.value(), 10.0);
    assert_eq!(centimeters.value(), 10.0);
    assert_eq!(kilometers.value(), 10.0);

    assert_eq!(f64::from(meters), 10.0);
    assert_eq!(Length::from(meters).get::<length::meter>(), 10.0);
}

#[test]
fn test_elv_macro_default_and_units() {
    assert_eq!(format!("{}", elv!(10.0)), "10 m");
    assert_eq!(format!("{}", elv!(10.0; cm)), "0.1 m");
    assert_eq!(format!("{}", elv!(0.01; km)), "10 m");
    assert_eq!(format!("{:#}", elv!(10.0; cm)), "10 centimeters");
}

#[test]
fn test_elevation_display_alternate_units() {
    assert_eq!(format!("{:#}", Elevation::meters(0.005)), "5 millimeters");
    assert_eq!(format!("{:#}", Elevation::meters(0.05)), "5 centimeters");
    assert_eq!(format!("{:#}", Elevation::meters(10.0)), "10 meters");
    assert_eq!(format!("{:#}", Elevation::kilometers(1.5)), "1.5 kilometers");
}

#[test]
fn test_elevation_try_from_invalid_values() {
    assert!(matches!(Elevation::try_from(f64::NAN), Err(Error::InvalidNumericValue(_))));
    assert!(matches!(Elevation::try_from(f64::INFINITY), Err(Error::InvalidNumericValue(_))));
}

#[test]
fn test_elevation_from_str() {
    let elevation = Elevation::from_str("123.45").unwrap();

    assert_eq!(elevation.value(), 123.45);
    assert_eq!(format!("{elevation}"), "123.45 m");
}

// --- CoordinateWithElevation construction and formatting ---

#[test]
fn test_coordinate_with_elevation_default_display() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let elev = Elevation::meters(100.0);
    let coord = CoordinateWithElevation::new_from(lat, lon, elev);

    assert_eq!(coord.point(), Coordinate::new(lat, lon));
    assert_eq!(coord.elevation(), elev);
    assert_eq!(format!("{coord}"), "51.50722222, 0.12750000, 100 m");
    assert_eq!(format!("{coord:#}"), "51° 30′ 26.000000″, 0° 7′ 39.000000″, 100 m");
}

#[test]
fn test_coordinate_with_elevation_accessors_and_modifiers() {
    let lat = Latitude::new(10, 0, 0.0).unwrap();
    let lon = Longitude::new(20, 0, 0.0).unwrap();
    let elev = Elevation::meters(5.0);
    let coord = CoordinateWithElevation::new_from(lat, lon, elev);

    let new_lat = Latitude::new(-10, 0, 0.0).unwrap();
    let new_lon = Longitude::new(-20, 0, 0.0).unwrap();
    let updated = coord
        .with_point(Coordinate::new(new_lat, new_lon))
        .with_elevation(Elevation::meters(50.0));

    assert_eq!(updated.point().latitude(), new_lat);
    assert_eq!(updated.point().longitude(), new_lon);
    assert_eq!(updated.elevation(), Elevation::meters(50.0));
    assert!(!updated.is_zero_elevation());
}

#[test]
fn test_coordinate_with_elevation_with_new_point() {
    let lat = Latitude::new(0, 0, 0.0).unwrap();
    let lon = Longitude::new(0, 0, 0.0).unwrap();
    let elev = Elevation::meters(0.0);
    let coord = CoordinateWithElevation::new_from(lat, lon, elev);

    let new = coord.with_new_point(Latitude::new(23, 30, 0.0).unwrap(), Longitude::new(45, 0, 0.0).unwrap());

    assert_eq!(new.point().latitude(), Latitude::new(23, 30, 0.0).unwrap());
    assert_eq!(new.point().longitude(), Longitude::new(45, 0, 0.0).unwrap());
    assert!(new.is_zero_elevation());
}

#[test]
fn test_coordinate_with_elevation_from_str_failure() {
    assert!(matches!(CoordinateWithElevation::from_str("not-a-coordinate"), Err(Error::InvalidCoordinate)));
}

#[test]
fn test_coordinate_with_elevation_hemisphere_predicates() {
    let lat = Latitude::new(-1, 0, 0.0).unwrap();
    let lon = Longitude::new(1, 0, 0.0).unwrap();
    let coord = CoordinateWithElevation::new_from(lat, lon, Elevation::zero());

    assert!(coord.is_southern());
    assert!(!coord.is_northern());
    assert!(coord.is_eastern());
    assert!(!coord.is_western());
    assert!(!coord.is_on_equator());
    assert!(!coord.is_on_international_reference_meridian());
}

#[test]
fn test_coordinate_with_elevation_equality_and_ordering() {
    let small = Elevation::meters(5.0);
    let large = Elevation::meters(10.0);

    assert!(small < large);
    assert_eq!(small, Elevation::from(small));
}
