# Rust wrapper around [libucl][libucl]

![](https://github.com/draft6/libucl-rs/workflows/Build/badge.svg)
[![MIT Licensed](https://img.shields.io/badge/Licence-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Usage

```rust
use libucl::Parser;

let parser = Parser::new();
let result = parser.parse(r#"tag = "svc";
upstream {
    h2c = true;
    host = "http://localhost";
    port = 9090;
    connect_timeout = 1s;
}"#).unwrap();

println!("{}", result.fetch_path("upstream.h2c").and_then(|v| v.as_bool()));
```

## Licence

Check out [LICENSE](LICENSE) file.

[libucl]: https://github.com/vstakhov/libucl "Universal configuration library parser"
