# AGENTS.md — lat-long

Context file for AI coding agents. Describes architecture, conventions, and
workflows relevant to this codebase.

## Project Overview

`lat-long` is a focused Rust library (`no_std`-compatible) providing strongly-typed
geographic coordinate primitives: `Latitude`, `Longitude`, and `Coordinate`.
Values are validated on construction and support both decimal-degrees and
degrees–minutes–seconds display/parsing.

- **Crate name:** `lat-long` (lib target: `lat_long`)
- **Version:** 0.1.3 — published to [crates.io](https://crates.io/crates/lat-long)
- **Edition:** 2024
- **MSRV:** Rust 1.90
- **License:** MIT OR Apache-2.0

### Source Layout

```text
src/
  lib.rs        # Crate root; re-exports public types; defines the `Angle` trait
  latitude.rs   # `Latitude` type, constants (NORTH_POLE, EQUATOR, …), `lat!` macro
  longitude.rs  # `Longitude` type, constants (PRIME_MERIDIAN, …), `lon!` macro
  coord.rs      # `Coordinate` (Latitude + Longitude pair)
  elevation.rs  # `Elevation` and `CoordinateWithElevation` — gated on feature `3d`
  fmt.rs        # `FormatOptions`, `FormatKind`, `Formatter` trait, formatting logic
  parse.rs      # `parse_str`, `Parsed` enum, `FromStr` impls
  error.rs      # `Error` enum (all validation & parse errors)
  inner.rs      # Private helpers: DMS ↔ decimal-degree conversions, `ZERO` const

tests/
  coordinate_tests.rs
  elevation_tests.rs
  latitude_tests.rs
  longitude_tests.rs
  parse_tests.rs
```

---

## Guidelines

### Code Style Guidelines

- Follow the existing module layout; one primary type per module.
- Use `#[must_use]` on pure query methods (see `Latitude::is_northern`, etc.).
- `inner` module is `pub(crate)` only — never expose it publicly.
- All public items require doc comments with at least one `# Examples` section.
- Macro naming: `lat!` / `lon!` — lowercase, matching the module name.
- Do **not** use `unwrap()` in library code; use typed errors + `?`.
- `no_std` compatibility: avoid importing from `std::` directly in feature-gated
  code; prefer `core::` and `alloc::` where possible.
- Format strings use Unicode typographic characters: `°` `′` `″` (not `'` `"`).

### Core Abstraction: the `Angle` Trait

`Angle` (defined in `src/lib.rs`) is the shared interface for `Latitude` and
`Longitude`. Any implementation must be:

- `Clone + Copy + Debug + Default + Display + PartialEq + Eq + PartialOrd + Ord + Hash`
- Convertible from/into `OrderedFloat<f64>` (provides NaN-free float comparison)
- Constructable via `Angle::new(degrees: i32, minutes: u32, seconds: f32) -> Result<Self, Error>`

Trait-provided methods include: `degrees()`, `minutes()`, `seconds()`,
`is_zero()`, `is_nonzero_positive()`, `is_nonzero_negative()`, `checked_abs()`,
`saturating_abs()`, `overflowing_abs()`, `wrapping_abs()`, `strict_abs()`,
`unchecked_abs()`.

## Error Handling

All errors are variants of `error::Error` and implement `Display` + `std::error::Error`.
Construction methods (`Angle::new`, `TryFrom<f64>`) return `Result<Self, Error>`.

Never use `.unwrap()` in library code. Use `.expect("reason")` only in test helpers
or doc examples that are guaranteed to succeed.

### Parsing

`parse::parse_str(s: &str) -> Result<Parsed, Error>` accepts all four format
kinds and returns a `Parsed` enum:

```rust
pub enum Parsed {
    Angle(Value),       // Value::Latitude, Value::Longitude, or Value::Unknown
    Coordinate(Coordinate),
}
```

`FromStr` is implemented for `Latitude`, `Longitude`, and `Coordinate` by
delegating to `parse_str`.

### Build & Workflow Commands

All standard workflows are driven through `make` (see `Makefile`).

| Goal          | Command                          | Notes                                                                          |
|---------------|----------------------------------|--------------------------------------------------------------------------------|
| Format        | `make format`                    | Runs `cargo fmt`                                                               |
| Lint          | `make clippy`                    | Runs `cargo clippy --workspace`                                                |
| Build matrix  | `make build`                     | Default, all-features, no-default-features × debug+release                     |
| Test matrix   | `make test`                      | Runs across default, all-features, no-default-features, and per-feature combos |
| Coverage      | `make coverage`                  | Runs `cargo tarpaulin` (see `.tarpaulin.toml`)                                 |
| Docs          | `make docs`                      | `cargo doc --all-features --no-deps`.                                          |
| Publish check | `make publish`                   | Dry-run; runs full lint+test+docs first                                        |

Always run `make test` before committing — never just `cargo test`.

### Testing Conventions

- Integration tests live in `tests/`; unit tests (if any) live in `#[cfg(test)]`
  modules inside `src/`.
- Test file naming: `<type>_tests.rs` (e.g., `latitude_tests.rs`).
- **Test behavior, not implementation.** Every test should answer: _"what user-visible
  behavior breaks if this test fails?"_
- Prefer testing:
  - Valid construction succeeds and round-trips correctly.
  - Out-of-range / invalid inputs return the expected `Error` variant.
  - `Display` output matches expected string exactly.
  - `FromStr` parses back to the original value.
  - Feature-gated code (serde, geojson, urn) is tested under the appropriate feature flag.
- Avoid tests that only assert a function was called or a mock was invoked.

### Coverage

Coverage is configured in `.tarpaulin.toml` with named profiles:

| Profile          | Description                              |
|------------------|------------------------------------------|
| `default_release`| Default features, release mode           |
| `all_features`   | All features, debug mode                 |
| `no_std`         | `--no-default-features --features alloc` |
| `default_serde`  | `--features serde`                       |
| `default_url`    | `--features url`                         |
| `geojson`        | `--features geojson`                     |

Output formats: HTML and XML (see `[report]` section).

### Publishing Checklist

1. Bump `version` in `Cargo.toml`.
2. Run `make all` (format → clippy → test → docs).
3. Run `make publish` (dry-run).
4. Tag the commit and push.
5. Run `cargo publish`.
