extern crate clap;
extern crate lisp;

use clap::{Arg, Command};
use std::{fs::canonicalize, io};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::new("input")
                .alias("input")
                .index(1)
                .required(false)
                .help("Sets the input file to use"),
        )
        .get_matches();

    let scope = lisp::new();
    if let Some(input_file) = matches.value_of("input") {
        lisp::run_path(
            &scope,
            &canonicalize(input_file).expect("Failed to canonicalize input file"),
        )
    } else {
        lisp::repl(&scope)
    }
}
