# Package `lat-long`

Simple types for representing latitude and longitude coordinates.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![crates.io](https://img.shields.io/crates/v/lat-long.svg)](https://crates.io/crates/lat-long)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-lat-long.svg)](<https://github.com/johnstonskj/rust-lat-long/stargazers>)

## Examples

Basic construction and display.

```rust
use lat_long::{Angle, Coordinate, Latitude, Longitude};

let lat = Latitude::new(48, 51, 29.6).expect("valid latitude");
let lon = Longitude::new(2, 21, 7.6).expect("valid longitude");
let paris = Coordinate::new(lat, lon);

// Decimal-degree display (default)
println!("{paris}");   // => 48.858222, 2.218778
// Degrees–minutes–seconds display (alternate flag)
println!("{paris:#}"); // => 48° 51' 29.6" N, 2° 21' 7.6" E
```

Parsing a coordinate from a string.

```rust
use lat_long::{parse::{self, Parsed}, Coordinate};

if let Ok(Parsed::Coordinate(london)) = parse::parse_str("51.522, -0.127") {
    println!("{london}"); // => 51.522, -0.127
}
```

Construct URL (URN) from coordinate.

```rust
// Convert to URL, requires `url` feature flag
let url = url::Url::from(paris);
println!("{url}"); // => geo:48.858222,2.218778
```

Construcxt a JSON value according to the GeoJSON spec.

```rust
// Convert to JSON, requires `geojson` feature flag
let json = serde_json::Value::from(paris);
println!("{json}"); // => { "type": "Point", "coordinates": [48.858222,2.218778] }
```

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-MIT).
