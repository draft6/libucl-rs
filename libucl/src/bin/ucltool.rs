extern crate clap;

use clap::{App, Arg};

fn main(){
    let matches = App::new("Ucl Tool")
    .arg(
        Arg::with_name("help")
             .help("print this message and exit\n"),
    ).arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .help("specify input filename (default standard input)")
                .takes_value(true)
                .value_name("INFILE")
        ).arg(
        Arg::with_name("out")
            .short("o")
            .long("out")
            .help("specify output filename (default standard output)")
            .takes_value(true)
            .value_name("OUTFILE")
    ).arg(
        Arg::with_name("schema")
            .short("s")
            .long("schema")
            .help("specify file for validation")
            .takes_value(true)
            .value_name("SCHEMA")
    ).arg(
        Arg::with_name("format")
            .long("format")
            .short("f")
            .takes_value(true)
            .possible_values(&["ucl","json","json_compact","yaml","msgpack"])
            .help("Output format, Options ucl, json, compact_json, yaml, msgpack")
    ).get_matches();

    if let Some(o) = matches.value_of("out") {
        println!("Value for output: {}", o);
    }

    if let Some(c) = matches.value_of("in") {
        println!("Value for config: {}", c);
    }

    if let Some(c) = matches.value_of("format") {
        println!("Value for schema: {}", c);
    }

}