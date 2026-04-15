use lat_long::{
    Angle, Coordinate, Latitude, Longitude,
    fmt::{FormatOptions, Formatter},
};

// --- Coordinate construction ---

#[test]
fn test_coord_valid_values() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    Coordinate::new(lat, lon);
}

// --- Display ---

#[test]
fn test_coord_display() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let london = Coordinate::new(lat, lon);

    assert_eq!(format!("{london}"), "51.50722222, 0.12750000");
}

#[test]
fn test_coord_display_alt() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let london = Coordinate::new(lat, lon);

    assert_eq!(
        format!("{london:#}"),
        "51° 30′ 26.000000″, 0° 7′ 39.000000″"
    );
}

// --- Formatting ---

#[test]
fn test_coord_format_decimal() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let london = Coordinate::new(lat, lon);

    assert_eq!(
        london.to_formatted_string(&FormatOptions::decimal().with_precision(2)),
        "51.51, 0.13"
    );
}

#[test]
fn test_coord_format_dms_signed() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let london = Coordinate::new(lat, lon);

    assert_eq!(
        london.to_formatted_string(&FormatOptions::dms_signed().with_precision(2)),
        "51° 30′ 26.00″, 0° 7′ 39.00″"
    );
}

#[test]
fn test_coord_format_dms_labeled() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let london = Coordinate::new(lat, lon);

    assert_eq!(
        london.to_formatted_string(&FormatOptions::dms_labeled().with_precision(2)),
        "51° 30′ 26.00″ N, 0° 7′ 39.00″ E"
    );
}

#[test]
fn test_coord_format_dms_bare() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let london = Coordinate::new(lat, lon);

    // DmsBare enforces a minimum of 4 decimal places for seconds; passing
    // precision=2 is clamped up to 4.
    assert_eq!(
        london.to_formatted_string(&FormatOptions::dms_bare().with_precision(2)),
        "+051:30:26.0000,+000:07:39.0000"
    );
}

// --- Hemisphere predicates ---

#[test]
fn test_coord_is_northern() {
    let north = Latitude::new(45, 0, 0.0).unwrap();
    assert!(north.is_northern());
    assert!(!north.is_southern());
    assert!(!north.is_on_equator());
}

#[test]
fn test_coord_is_on_equator() {
    let equator = Latitude::new(0, 0, 0.0).unwrap();
    assert!(equator.is_on_equator());
    assert!(!equator.is_northern());
    assert!(!equator.is_southern());
}

#[test]
fn test_coord_is_southern() {
    let south = Latitude::new(-45, 0, 0.0).unwrap();
    println!("{south}");
    assert!(south.is_southern());
    assert!(!south.is_northern());
    assert!(!south.is_on_equator());
}

// --- Zone predicates ---

#[test]
fn test_coord_is_polar() {
    let lat = Latitude::new(70, 0, 0.0).unwrap();
    let lon = Longitude::new(25, 0, 0.0).unwrap();
    let c = Coordinate::new(lat, lon);
    // 70°N is above the Arctic Circle — delegate matches Latitude::is_polar.
    assert!(c.latitude().is_polar());
    assert!(c.latitude().is_arctic());
}

#[test]
fn test_coord_is_tropical() {
    // is_tropical() is true at-or-beyond the Tropic of Cancer (>= 23.5 N)
    // or at-or-beyond the Tropic of Capricorn (<= 23.5 S).
    let lat = Latitude::new(30, 0, 0.0).unwrap();
    let lon = Longitude::new(100, 0, 0.0).unwrap();
    let c = Coordinate::new(lat, lon);
    assert!(c.latitude().is_tropical()); // 30 N >= 23.5 N
    assert!(!c.latitude().is_polar());

    // 15 N is between the tropics: is_tropical() is false.
    let bt = Coordinate::new(Latitude::new(15, 0, 0.0).unwrap(), lon);
    assert!(!bt.latitude().is_tropical());
}

// --- Accessors ---

#[test]
fn test_coord_accessors() {
    let lat = Latitude::new(51, 30, 26.0).unwrap();
    let lon = Longitude::new(0, 7, 39.0).unwrap();
    let london = Coordinate::new(lat, lon);
    assert_eq!(london.latitude(), lat);
    assert_eq!(london.longitude(), lon);
    // Greek-letter aliases
    assert_eq!(london.φ(), lat);
    assert_eq!(london.λ(), lon);
}

#[test]
fn test_coord_with_latitude() {
    let lat1 = Latitude::new(51, 30, 0.0).unwrap();
    let lat2 = Latitude::new(48, 51, 30.0).unwrap();
    let lon = Longitude::new(2, 21, 8.0).unwrap();
    let london = Coordinate::new(lat1, lon);
    let paris = london.with_latitude(lat2);
    assert_eq!(paris.latitude(), lat2);
    assert_eq!(paris.longitude(), lon); // longitude unchanged
}

#[test]
fn test_coord_with_longitude() {
    let lat = Latitude::new(51, 30, 0.0).unwrap();
    let lon1 = Longitude::new(0, 7, 39.0).unwrap();
    let lon2 = Longitude::new(2, 21, 8.0).unwrap();
    let london = Coordinate::new(lat, lon1);
    let paris = london.with_longitude(lon2);
    assert_eq!(paris.latitude(), lat); // latitude unchanged
    assert_eq!(paris.longitude(), lon2);
}

#[test]
fn test_coord_to_url_string() {
    let lat = Latitude::new(48, 51, 30.0).unwrap();
    let lon = Longitude::new(2, 21, 8.0).unwrap();
    let paris = Coordinate::new(lat, lon);
    let url = paris.to_url_string();
    assert!(url.starts_with("geo:"), "expected geo: prefix, got {url}");
    // The string should contain a comma separating the two decimal values.
    let body = url.strip_prefix("geo:").unwrap();
    let parts: Vec<&str> = body.split(',').collect();
    assert_eq!(parts.len(), 2, "expected exactly two parts, got {url}");
}

#[cfg(feature = "serde")]
mod serde {
    use lat_long::{Angle, Coordinate, Latitude, Longitude};

    #[test]
    fn test_serialize_coordinate() {
        let lat = Latitude::new(48, 51, 30.0).unwrap();
        let lon = Longitude::new(2, 21, 8.0).unwrap();
        let paris = Coordinate::new(lat, lon);

        assert_eq!(
            r##"{
  "lat": 48.858333333333334,
  "long": 2.352222222222222
}"##
            .to_string(),
            serde_json::to_string_pretty(&paris).unwrap()
        );
    }
}
