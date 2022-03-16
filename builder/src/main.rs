extern crate clap;
extern crate lisp_builder;

use clap::{Arg, Command};
use std::fs;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .default_value("./Cargo.toml")
                .help("Sets the input Cargo.toml to use"),
        )
        .arg(
            Arg::new("dest")
                .short('d')
                .long("dest")
                .default_value("./lisp-builder-out")
                .help("Sets the output path to use"),
        )
        .get_matches();

    let dest_dir = matches.value_of("dest").unwrap_or("./dest");
    fs::create_dir_all(dest_dir).expect("Failed to create dest directory");

    lisp_builder::build(
        &fs::canonicalize(matches.value_of("input").unwrap_or("."))
            .expect("Failed to canonicalize input file"),
        &fs::canonicalize(dest_dir).expect("Failed to canonicalize output file"),
    );
}
