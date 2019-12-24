extern crate clap;
extern crate libucl;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::prelude::*;
use std::path::Path;

use clap::{App, Arg};

fn main() {
    let matches = App::new("UCL Tool")
        .arg(
            Arg::with_name("help")
                .help("print this message and exit\n"),
        ).arg(
        Arg::with_name("in")
            .short("i")
            .long("in")
            .help("Specify input filename path (defaults to standard input)")
            .takes_value(true)
            .value_name("INFILE")
    ).arg(
        Arg::with_name("out")
            .short("o")
            .long("out")
            .help("Specify output filename path(defaults to standard output)")
            .takes_value(true)
            .value_name("OUTFILE")
    ).arg(
        Arg::with_name("schema")
            .short("s")
            .long("schema")
            .help("Specify schema file path to perform validation")
            .takes_value(true)
            .value_name("SCHEMA")
    ).arg(
        Arg::with_name("format")
            .long("format")
            .short("f")
            .takes_value(true)
            .default_value("ucl")
            .possible_values(&["ucl", "json", "json_compact", "yaml", "msgpack"])
            .help("Specify the output format")
    ).get_matches();

    let format = match matches.value_of("format") {
        Some(t) => match t {
            "json" => libucl::Emitter::JSON,
            "json_compact" => libucl::Emitter::JSONCompact,
            "yaml" => libucl::Emitter::YAML,
            "msgpack" => libucl::Emitter::MsgPack,
            "ucl" => libucl::Emitter::Config,
            _ => libucl::Emitter::Config
        },
        None => libucl::Emitter::Config
    };
    let content = match matches.value_of("in") {
        Some(filename) => {
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");
            contents
        },
        None => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buffer).expect("Something went wrong while reading from the standard input");
            buffer
        }
    };
    let parser = libucl::Parser::new();
    let content = parser.parse(content).unwrap();

    if let Some(filename) = matches.value_of("scheme") {
        let schema = fs::read_to_string(filename)
            .expect("Something went wrong while reading the file");
        let parser = libucl::Parser::new();
        let schema = parser.parse(schema).unwrap();
        content.validate_with_schema(&schema).unwrap();
    }
    match matches.value_of("out") {
        Some(filename) => {
            let path = Path::new(filename);
            let display = path.display();
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                Ok(file) => file,
            };
            match file.write_all(content.dump_into(format).as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
                Ok(_) => println!("successfully wrote to {}", display),
            }
        },
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(content.dump_into(format).as_bytes())
                .expect("Error writing to stdout");
        }
    };
}