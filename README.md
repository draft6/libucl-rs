# Rust wrapper around [libucl][libucl]

![](https://github.com/draft6/libucl-rs/workflows/Build/badge.svg)
[![MIT Licensed](https://img.shields.io/badge/Licence-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/libucl)](https://crates.io/crates/libucl)
A lightweight wrapper library in Rust around libucl, a library used for parsing of UCL (Universal Configuration Language) files.

## Platform support
Linux / Mac OSX
## Basics
You can read all about UCL (Universal Configuration Language) [here][libucldoc] 
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

## Validation
You can write validation schemas in UCL format as well,
 as long as it follows the JSON Schema rules for defining a schema with the exception of remote references.
 UCL currently is not absolutely strict about validation schemas themselves, 
 therefore UCL users should supply valid schemas (as it is defined in json-schema draft v4) to ensure that the input objects are validated properly.
```rust
use libucl::Parser;

let parser = Parser::new();
let item = parser.parse(r#"
    {
    "key": "some string"
    }"#
).unwrap();

let parser = Parser::new();
let schema = parser.parse(r#"
    {
    "type": "object",
     "properties":{
        "key": {
            "type":"string"
            }
        }
    }"#
    ).unwrap();
let res = item.validate_with_schema(&schema);
assert_eq!(res.is_ok(), true);

```
## Dump Object
It's possible to dump objects into JSON, JSON compact, YAML and Config format

```rust
  let parser = Parser::new();
  let result = parser.parse(r#"section {
    flag = true;
    number = 10k;
    subsection {
        hosts = {
            host = "localhost";
            port = 9000
        }
        hosts = {
            host = "remotehost"
            port = 9090
        }
    }
}"#).unwrap();
        let regex = Regex::new("\"flag\":true").unwrap();
        let val = result.dump_into(Emitter::JSONCompact);
        assert_eq!(regex.is_match(val.as_str()), true);


```

## Licence

Check out [LICENSE](LICENSE) file.

[libucl]: https://github.com/vstakhov/libucl "Universal configuration library parser"
[libucldoc]: https://github.com/vstakhov/libucl#introduction "UCL introduction"
